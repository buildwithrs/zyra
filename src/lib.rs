pub mod errors;
pub mod lexer;
pub mod token;

pub use errors::ZyraErrors;
pub use lexer::Lexer;
pub use token::Token;

pub type ZyraResult<T> = Result<T, ZyraErrors>;
