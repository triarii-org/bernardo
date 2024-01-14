use crate::*;
use crate::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LspIOError {
    Write(LspWriteError),
    Read(LspReadError),
}

impl From<LspReadError> for LspIOError {
    fn from(r: LspReadError) -> Self {
        LspIOError::Read(r)
    }
}

impl From<LspWriteError> for LspIOError {
    fn from(w: LspWriteError) -> Self {
        LspIOError::Write(w)
    }
}
