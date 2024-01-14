use crate::*;
use crate::*;

#[derive(Clone, Debug)]
pub enum SaveFileDialogMsg {
    FocusUpdateMsg(FocusUpdate),
    // Sent when a left hand-side file-tree subtree is expanded (default: on Enter key)
    TreeExpanded(SPath),
    // Sent when a left hand-side file-tree subtree selection changed
    TreeHighlighted(SPath),

    TreeHit(SPath),

    FileListHit(SPath),
    EditBoxHit,

    Cancel,
    Save,

    ConfirmOverride,
    CancelOverride,
}

impl AnyMsg for SaveFileDialogMsg {}
