// TODO: This file is very similar to ReadError in FS. Maybe it's worth merging them?
// They are however in one way distinct: we want to be able to Load Config from outside FS.

use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use crate::*;

#[derive(Debug)]
pub enum ConfigLoadError {
    ReadError(ReadError),
    IoError(std::io::Error),
    DeserializationError(ron::Error),
    UnmappedError(String),
}

impl From<ron::Error> for ConfigLoadError {
    fn from(e: ron::Error) -> Self {
        ConfigLoadError::DeserializationError(e)
    }
}

impl From<ReadError> for ConfigLoadError {
    fn from(re: ReadError) -> Self {
        ConfigLoadError::ReadError(re)
    }
}

impl From<std::io::Error> for ConfigLoadError {
    fn from(ioe: std::io::Error) -> Self {
        ConfigLoadError::ReadError(ReadError::from(ioe))
    }
}

impl From<std::str::Utf8Error> for ConfigLoadError {
    fn from(ue: Utf8Error) -> Self {
        ConfigLoadError::ReadError(ReadError::Utf8Error(ue))
    }
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

//TODO
impl From<ron::error::SpannedError> for ConfigLoadError {
    fn from(e: ron::error::SpannedError) -> Self {
        ConfigLoadError::UnmappedError(format!("{}", e))
    }
}

impl std::error::Error for ConfigLoadError {}
