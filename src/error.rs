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