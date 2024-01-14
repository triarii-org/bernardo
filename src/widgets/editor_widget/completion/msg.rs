use crate::*;
use crate::*;

#[derive(Clone, Debug)]
pub enum CompletionWidgetMsg {
    Close,
    Selected(CompletionAction),
}

impl AnyMsg for CompletionWidgetMsg {}
