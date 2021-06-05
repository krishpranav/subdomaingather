use std::error::Error;
use std::fmt::{self, Formatter};

pub type Result<T> = std::result::Result<T, SubError>;