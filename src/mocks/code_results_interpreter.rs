use crate::*;

pub struct CodeResultsViewInterpreter<'a> {
    meta: &'a Metadata,
    output: &'a MetaOutputFrame,
}

impl<'a> CodeResultsViewInterpreter<'a> {
    pub fn new(output: &'a MetaOutputFrame, meta: &'a Metadata) -> Self {
        debug_assert!(meta.typename == CodeResultsView::TYPENAME);

        Self { meta, output }
    }

    pub fn is_focused(&self) -> bool {
        self.meta.focused
    }

    // pub fn contents(&self) -> String {
    //     self.output.buffer.lines_iter().with_rect(self.meta.rect).next().unwrap().text.trim().
    // to_string() }
}
