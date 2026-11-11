pub mod ast; // define ABstract Syntax Tree
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod token; // the INterpreter Parser from tokens into Statement and Expression

pub use errors::ZyraErrors;
pub use lexer::Lexer;
pub use token::KW;
pub use token::Token;

pub use errors::ParserErrors;

pub type ZyraResult<T> = Result<T, ZyraErrors>;
