use crate::*;
use crate::*;

#[derive(Clone, Debug)]
pub enum ContextBarWidgetMsg {
    Close,
    Edit(CommonEditMsg),
    Hit,
}

impl AnyMsg for ContextBarWidgetMsg {}
