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
            SubError::SourceError(s) => write!(f, "couldn't fetch data from {}", s),
            SubError::AuthError(s) => {
                write!(
                    f,
                    "error authenticating to {} or may have hit rate limits",
                    s
                )
            }
            SubError::UnsetKeys(v) => write!(f, "error reading environment variables {:?}", v),
            SubError::EmptyResults => write!(f, "returned no results"),
            SubError::CrobatError => {
                write!(f, "got error when trying to pull results from crobat")
            }
            SubError::ParseError => write!(f, "got error trying to parse cli args"),
            SubError::Msg(s) => write!(f, "got error {}", s),
            SubError::ReqwestError(ref err) => err.fmt(f),
            SubError::JoinError(ref err) => err.fmt(f),
            SubError::IoError(ref err) => err.fmt(f),
        }
    }
}

impl Error for SubError{}

impl From<String> for SubError {
    fn from(err: String) -> Self {
        SubError::Msg(err)
    }
}

impl From<reqwest::Error> for SubError {
    fn from(err: reqwest::Error) -> Self {
        SubError::ReqwestError(err)
    }
}

impl From<tokio::task::JoinError> for SubError {
    fn from(err: tokio::task::JoinError) -> Self {
        SubError::JoinError(err)
    }
}

impl From<std::io::Error> for SubError {
    fn from(err: std::io::Error) -> Self {
        SubError::IoError(err)
    }
}