use std::fmt::Debug;

use crate::fs::path::SPath;
use crate::widgets::main_view::main_view::DocumentIdentifier;
use crate::*;

#[derive(Debug)]
pub enum CodeResultsMsg {
    Hit,
}

impl AnyMsg for CodeResultsMsg {}
