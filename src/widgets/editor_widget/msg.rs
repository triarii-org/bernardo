use std::fmt::Debug;

use crate::*;
use crate::*;

use crate::*;

#[derive(Clone, Debug)]
pub enum EditorWidgetMsg {
    EditMsg(CommonEditMsg),

    ToCursorDropMode,
    ToEditMode,

    DropCursorFlip { cursor: Cursor },
    // not sure if this can't be simplified, why separate message?
    DropCursorMove { cem: CommonEditMsg },

    OpenContextMenu,
    ContextMenuClose,

    RequestCompletions,
    HoverClose,
    CompletionWidgetSelected(CompletionAction),

    RequestContextBar,

    Reformat,
    GoToDefinition,
    ShowUsages,
}

impl AnyMsg for EditorWidgetMsg {}
