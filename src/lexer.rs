use crate::{Token, ZyraResult, errors::LexerErrors, token::KW};

pub struct Lexer {
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
}

impl Lexer {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            start: 0,
            current: 0,
        }
    }

    pub fn lex(&mut self) -> ZyraResult<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            if self.is_end() {
                break;
            }

            self.start = self.current;
            let tk = self.next_token()?;
            match tk {
                Token::Whitespace => {}
                _ => tokens.push(tk),
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> ZyraResult<Token> {
        let cur = self.advance().ok_or_else(|| LexerErrors::NOChars)?;
        match cur {
            '\n' | '\t' | ' ' => Ok(Token::Whitespace),
            '+' => {
                if self.is_match('+') {
                    self.advance();
                    Ok(Token::Increment)
                } else {
                    Ok(Token::Plus)
                }
            }
            '-' => {
                if self.is_match('-') {
                    self.advance();
                    Ok(Token::Decrement)
                } else {
                    Ok(Token::Minus)
                }
            }
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '%' => Ok(Token::Mod),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),

            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semi),

            '&' => {
                if self.is_match('&') {
                    self.advance();
                    Ok(Token::And)
                } else {
                    Err(LexerErrors::UnknowChar('&').into())
                }
            }

            '=' => {
                if self.is_match('=') {
                    self.advance();
                    Ok(Token::Eq)
                } else {
                    Ok(Token::Assign)
                }
            }

            '|' => {
                if self.is_match('|') {
                    self.advance();
                    Ok(Token::Or)
                } else {
                    Err(LexerErrors::UnknowChar('|').into())
                }
            }

            '>' => {
                if self.is_match('=') {
                    self.advance();
                    Ok(Token::GreatThanEq)
                } else {
                    Ok(Token::GreatThan)
                }
            }

            '<' => {
                if self.is_match('=') {
                    self.advance();
                    Ok(Token::LessThanEq)
                } else {
                    Ok(Token::LessThan)
                }
            }

            '!' => {
                if self.is_match('=') {
                    self.advance();
                    Ok(Token::NotEq)
                } else {
                    Ok(Token::Bang)
                }
            }
            '"' => self.parse_string(),
            _ => {
                if cur.is_ascii_digit() {
                    self.parse_number()
                } else if cur.is_alphabetic() {
                    self.parse_ident()
                } else {
                    Err(LexerErrors::UnknowChar(cur).into())
                }
            }
        }
    }

    fn parse_number(&mut self) -> ZyraResult<Token> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
                continue;
            }
            break;
        }

        let num_text = String::from_iter(&self.chars[self.start..self.current]);
        match num_text.parse::<i64>() {
            Ok(n) => Ok(Token::Num(n)),
            Err(e) => Err(LexerErrors::InvalidNumber(e).into()),
        }
    }

    fn parse_string(&mut self) -> ZyraResult<Token> {
        let mut end = false;
        while let Some(ch) = self.peek() {
            if ch == '"' {
                end = true;
                break;
            }
            self.advance();
        }

        if !end && self.is_end() {
            return Err(LexerErrors::UnterminatedString.into());
        }

        self.advance(); // skip right "
        Ok(Token::Str(String::from_iter(
            &self.chars[self.start + 1..self.current - 1],
        )))
    }

    fn parse_ident(&mut self) -> ZyraResult<Token> {
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch.eq(&'_') {
                self.advance();
                continue;
            }
            break;
        }

        let text = String::from_iter(&self.chars[self.start..self.current]);
        match text.as_str() {
            "fn" => Ok(Token::KW(KW::Function)),
            "if" => Ok(Token::KW(KW::Function)),
            "else" => Ok(Token::KW(KW::Else)),
            "for" => Ok(Token::KW(KW::For)),
            "return" => Ok(Token::KW(KW::Return)),
            "print" => Ok(Token::KW(KW::Print)),
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Ok(Token::Ident(text)),
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn is_match(&self, ch: char) -> bool {
        self.peek().is_some_and(|v| v == ch)
    }

    fn peek(&self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            Some(self.chars[self.current])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            self.current += 1;
            Some(self.chars[self.current - 1])
        }
    }
}
