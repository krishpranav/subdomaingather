use std::error::Error;
use std::fmt::{self, Formatter};

pub type Result<T> = std::result::Result<T, SubError>;

#[derive(Debug)]
pub enum SubError {
    SourceError(String),
    AuthError(String),
    UnsetKeys(Sub<String>),
    ReqwestError(reqwest::Error),
    JoinError(tokio::task::JoinError),
    IoError(std::io::Error),
    Msg(String),
    ParseError,
    CrobatError,
    EmptyResults,
}

impl fmt::Display for SubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SubError::SourceError(s) => write!(f, "couldn't fetch data from {}", s)
            SubError::AuthError(s) => {
                write!(
                    f,
                    "error authentication to {} or may have hit rate limits",
                    s
                )
            }
            
        }
    }
}