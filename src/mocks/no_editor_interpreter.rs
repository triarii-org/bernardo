use crate::*;

pub struct NoEditorInterpreter<'a> {
    meta: &'a Metadata,
    output: &'a MetaOutputFrame,
}

impl<'a> NoEditorInterpreter<'a> {
    pub fn new(output: &'a MetaOutputFrame, meta: &'a Metadata) -> Self {
        debug_assert!(meta.typename == NoEditorWidget::TYPENAME);

        Self { meta, output }
    }

    pub fn is_focused(&self) -> bool {
        self.meta.focused
    }
}
