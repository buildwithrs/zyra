#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub stmt: Stmt,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub cond: Box<Expr>,
    pub then: Block,
    pub else_branch: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub var_init: Box<Expr>,
    pub cond: Box<Expr>,
    pub body: Block,
    pub update: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fn {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
    Block(Block),
    Func(Fn),
    If(If),
    For(For),
    Return(Return),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Number(i64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
    NotEq,

    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Literal(Literal),

    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        value: Box<Expr>,
    },
    FnCall {
        ident: Box<Expr>,
        params: Vec<Expr>,
    },

    /// ++
    Increment {
        target: Box<Expr>,
    },
    /// --
    Decrement {
        target: Box<Expr>,
    },

    // arr[i]
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}
