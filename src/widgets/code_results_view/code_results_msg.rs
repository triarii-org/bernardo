use std::fmt::Debug;

use crate::fs::path::SPath;
use crate::*;

#[derive(Debug)]
pub enum CodeResultsMsg {
    Hit,
}

impl AnyMsg for CodeResultsMsg {}
