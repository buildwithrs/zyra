use crate::{
    KW::{self},
    ParserErrors,
    Token::{self},
    ZyraResult,
    ast::{
        BinOp, Block, Fn, For, If, Literal, Node, Program, Return,
        Stmt::{self},
        UnaryOp,
    },
};

use crate::ast::Expr;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ZyraResult<Program> {
        let mut nodes = Vec::new();
        loop {
            if self.is_end() {
                break;
            }

            let stmt = self.parse_stmt()?;
            nodes.push(Node {
                stmt,
                line: 0,
                col: 0,
            });
        }
        Ok(Program { nodes })
    }

    fn parse_stmt(&mut self) -> ZyraResult<Stmt> {
        let cur = self.peek().ok_or_else(|| ParserErrors::Eof)?;
        match cur {
            Token::KW(KW::Function) => self.parse_function(),
            Token::KW(KW::If) => self.parse_if(),
            Token::KW(KW::Break) => self.parse_break(),
            Token::KW(KW::Continue) => self.parse_continue(),
            Token::KW(KW::For) => self.parse_for(),
            Token::KW(KW::Return) => self.parse_return(),
            Token::LeftBrace => Ok(Stmt::Block(self.parse_block()?)),
            _ => {
                let expr = self.parse_expression()?;
                self.consume(Token::Semi)?;
                Ok(Stmt::Expr(Box::new(expr)))
            }
        }
    }

    fn parse_function(&mut self) -> ZyraResult<Stmt> {
        self.advance();
        let fn_name = self.advance().ok_or_else(|| ParserErrors::Eof)?;
        let name = match fn_name {
            Token::Ident(name) => name.clone(),
            _ => {
                return Err(
                    ParserErrors::ExpectToken(format!("expect ident token for fn name")).into(),
                );
            }
        };

        self.consume(Token::LeftParen)?;

        let mut params = Vec::new();
        while self.peek().is_some_and(|v| !v.eq(&Token::RightParen)) {
            if let Some(Token::Ident(name)) = self.peek() {
                params.push(name);
                self.advance();
            }
            if self.is_match(Token::RightParen) {
                break;
            }

            self.consume(Token::Comma)?;
        }
        self.consume(Token::RightParen)?;
        println!("consumed fn name and params: {:?}, {:?}", name, params);
        let block = self.parse_block()?;

        Ok(Stmt::Func(Fn {
            name,
            params,
            body: block,
        }))
    }

    fn parse_if(&mut self) -> ZyraResult<Stmt> {
        self.advance();
        let cond = self.parse_expression()?;
        let then = self.parse_block()?;
        let mut else_branch = None;

        if self.is_match(Token::KW(KW::Else)) {
            self.advance();
            else_branch = Some(self.parse_block()?);
        }

        Ok(Stmt::If(If {
            cond: Box::new(cond),
            then,
            else_branch,
        }))
    }

    fn parse_break(&mut self) -> ZyraResult<Stmt> {
        self.advance();
        self.consume(Token::Semi)?;
        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> ZyraResult<Stmt> {
        self.advance();
        self.consume(Token::Semi)?;
        Ok(Stmt::Continue)
    }

    fn parse_for(&mut self) -> ZyraResult<Stmt> {
        self.advance();
        let var_init = Box::new(self.parse_expression()?);
        self.consume(Token::Semi)?;

        let cond = Box::new(self.parse_expression()?);
        self.consume(Token::Semi)?;

        let update = Box::new(self.parse_expression()?);
        let body = self.parse_block()?;

        Ok(Stmt::For(For {
            var_init,
            cond,
            body,
            update,
        }))
    }
    fn parse_return(&mut self) -> ZyraResult<Stmt> {
        self.advance();

        if self.is_match(Token::Semi) {
            self.advance();
            Ok(Stmt::Return(Return { value: None }))
        } else {
            let val = self.parse_expression()?;
            self.consume(Token::Semi)?;
            Ok(Stmt::Return(Return { value: Some(val) }))
        }
    }
    fn parse_block(&mut self) -> ZyraResult<Block> {
        self.consume(Token::LeftBrace)?;
        let mut stmts = Vec::new();
        loop {
            if self.is_end() || self.is_match(Token::RightBrace) {
                break;
            }
            stmts.push(self.parse_stmt()?);
        }
        self.consume(Token::RightBrace)?;

        Ok(Block { stmts })
    }

    fn parse_expression(&mut self) -> ZyraResult<Expr> {
        let target = self.parse_or()?;
        if !self.is_match(Token::Assign) {
            return Ok(target);
        }

        self.consume(Token::Assign)?;

        if !matches!(target, Expr::Ident(_) | Expr::Index { .. }) {
            return Err(ParserErrors::ExpectToken(format!(
                "expect assign target to be ident or index"
            ))
            .into());
        }

        Ok(Expr::Assign {
            target: Box::new(target),
            value: Box::new(self.parse_expression()?),
        })
    }

    fn parse_or(&mut self) -> ZyraResult<Expr> {
        let mut left = self.parse_and()?;
        loop {
            if self.is_end() || !self.is_match(Token::Or) {
                break;
            }

            self.consume(Token::Or)?;
            let right = self.parse_and()?;
            left = Expr::Binary {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> ZyraResult<Expr> {
        let mut left = self.parse_equlity()?;
        loop {
            if self.is_end() || !self.is_match(Token::And) {
                break;
            }

            self.consume(Token::And)?;

            let right = Box::new(self.parse_equlity()?);
            left = Expr::Binary {
                op: BinOp::And,
                left: Box::new(left),
                right,
            };
        }
        Ok(left)
    }

    fn parse_equlity(&mut self) -> ZyraResult<Expr> {
        let mut left = self.parse_compare()?;
        if self.matches(vec![Token::Eq, Token::NotEq]) {
            let op = match self.advance() {
                Some(Token::Eq) => BinOp::Eq,
                Some(Token::NotEq) => BinOp::NotEq,
                _ => return Ok(left),
            };
            let right = Box::new(self.parse_compare()?);
            return Ok(Expr::Binary {
                op,
                left: Box::new(left),
                right,
            });
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> ZyraResult<Expr> {
        let left = self.parse_term()?;
        if self.matches(vec![
            Token::LessThan,
            Token::LessThanEq,
            Token::GreatThan,
            Token::GreatThanEq,
        ]) {
            let op = match self.advance() {
                Some(Token::LessThan) => BinOp::Lt,
                Some(Token::GreatThan) => BinOp::Gt,
                Some(Token::LessThanEq) => BinOp::LtEq,
                Some(Token::GreatThanEq) => BinOp::GtEq,
                _ => return Ok(left),
            };

            let right = Box::new(self.parse_term()?);
            return Ok(Expr::Binary {
                op,
                left: Box::new(left),
                right,
            });
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> ZyraResult<Expr> {
        let mut left = self.parse_factor()?;
        if self.is_match(Token::Plus) || self.is_match(Token::Minus) {
            let op = match self.advance().unwrap() {
                Token::Plus => BinOp::Plus,
                Token::Minus => BinOp::Minus,
                _ => return Ok(left),
            };

            Ok(Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(self.parse_factor()?),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_factor(&mut self) -> ZyraResult<Expr> {
        let left = self.parse_unary()?;
        if self.is_match(Token::Mul) || self.is_match(Token::Div) || self.is_match(Token::Mod) {
            let op = match self.advance().unwrap() {
                Token::Mul => BinOp::Mul,
                Token::Div => BinOp::Div,
                Token::Mod => BinOp::Mod,
                _ => return Ok(left),
            };

            Ok(Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(self.parse_unary()?),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> ZyraResult<Expr> {
        if self.is_match(Token::Plus) || self.is_match(Token::Minus) || self.is_match(Token::Bang) {
            let op = match self.advance() {
                Some(Token::Plus) => UnaryOp::Plus,
                Some(Token::Minus) => UnaryOp::Minus,
                Some(Token::Bang) => UnaryOp::Bang,
                _ => return Ok(self.parse_postfix()?),
            };

            return Ok(Expr::Unary {
                op,
                value: Box::new(self.parse_postfix()?),
            });
        }

        Ok(self.parse_postfix()?)
    }

    fn parse_postfix(&mut self) -> ZyraResult<Expr> {
        let pri = self.parse_primary()?;
        if self.is_match(Token::Increment)
            || self.is_match(Token::Decrement)
            || self.is_match(Token::LeftParen)
        {
            match self.advance() {
                Some(Token::Increment) => Ok(Expr::Increment {
                    target: Box::new(pri),
                }),
                Some(Token::Decrement) => Ok(Expr::Decrement {
                    target: Box::new(pri),
                }),
                Some(Token::LeftParen) => {
                    let mut params = Vec::new();
                    while self.peek().is_some_and(|v| !v.eq(&Token::RightParen)) {
                        params.push(self.parse_expression()?);
                        if !self.is_match(Token::RightParen) {
                            self.consume(Token::Comma)?;
                        }
                    }
                    self.consume(Token::RightParen)?;
                    Ok(Expr::FnCall {
                        ident: Box::new(pri),
                        params,
                    })
                }
                _ => Ok(pri),
            }
        } else {
            Ok(pri)
        }
    }

    fn parse_primary(&mut self) -> ZyraResult<Expr> {
        let cur = self.advance().ok_or_else(|| ParserErrors::Eof)?;
        match cur {
            Token::Str(s) => Ok(Expr::Literal(Literal::Str(s.clone()))),
            Token::Ident(id) => Ok(Expr::Ident(id.clone())),
            Token::Num(num) => Ok(Expr::Literal(Literal::Number(num))),
            Token::Null => Ok(Expr::Literal(Literal::Null)),
            Token::Boolean(b) => Ok(Expr::Literal(Literal::Boolean(b))),
            Token::KW(KW::Print) => Ok(Expr::Ident("print".to_string())),
            _ => Err(ParserErrors::ExpectToken(format!("expect literal tokens")).into()),
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.is_end() {
            None
        } else {
            Some(self.tokens[self.current].clone())
        }
    }

    pub fn is_match(&self, tk: Token) -> bool {
        self.peek().is_some_and(|v| v.eq(&tk))
    }

    fn consume(&mut self, tk: Token) -> ZyraResult<()> {
        if self.is_match(tk.clone()) {
            self.advance();
            return Ok(());
        }

        println!("expect: {:?}", tk.clone());
        Err(ParserErrors::TokenNotMatch.into())
    }

    fn matches(&self, tokens: Vec<Token>) -> bool {
        for tk in tokens {
            if self.is_match(tk) {
                return true;
            }
        }

        false
    }

    pub fn advance(&mut self) -> Option<Token> {
        if self.is_end() {
            None
        } else {
            self.current += 1;
            Some(self.tokens[self.current - 1].clone())
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
