use std::fmt::Debug;

use crate::*;

#[derive(Debug)]
pub enum CodeResultsMsg {
    Hit,
}

impl AnyMsg for CodeResultsMsg {}
