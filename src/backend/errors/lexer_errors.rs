use thiserror::Error;
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("ln:{line},ch:{char} -> Unknown token: {wrong_token}")]
    UnknownToken { wrong_token: String,line:usize,char:usize },
    #[error("ln:{line},ch:{char} -> Cannot have more dots in a number")]
    MoreDotInANumber{line:usize,char:usize},
    #[error("Unterminated string:{text}")]
    UnterminatedString{text:String},
    #[error("Cannot parse empty file")]
    EmptyFile,
}
