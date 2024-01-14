use crate::w7e::navcomp_provider::CompletionAction;
use crate::*;

#[derive(Clone, Debug)]
pub enum CompletionWidgetMsg {
    Close,
    Selected(CompletionAction),
}

impl AnyMsg for CompletionWidgetMsg {}
