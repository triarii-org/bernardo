use std::fmt::Debug;

use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum Navigation {
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
}

#[derive(Clone, Debug)]
pub enum FuzzySearchMsg {
    EditMsg(CommonEditMsg),
    EscalateContext,
    Navigation(Navigation),
    Hit,
    Close,
}

impl AnyMsg for FuzzySearchMsg {}
