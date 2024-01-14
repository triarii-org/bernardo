use crate::*;

/*
No time to come up with good name. It's basically a frame with "metadata" that was emited while it
was rendered.
 */
#[derive(Clone, Debug)]
pub struct MetaOutputFrame {
    pub buffer: BufferOutput,
    pub metadata: Vec<Metadata>,
    pub theme: Theme,
}

impl MetaOutputFrame {
    pub fn get_meta_by_type(&self, typename: &'static str) -> impl Iterator<Item = &Metadata> {
        self.metadata.iter().filter(move |i| i.typename == typename)
    }

    pub fn get_editors(&self) -> impl Iterator<Item = EditorInterpreter> {
        self.get_meta_by_type(EditorView::TYPENAME)
            .map(|meta| EditorInterpreter::new(self, meta))
            .flatten()
    }

    pub fn get_scroll<T: Widget>(&self) -> impl Iterator<Item = WithScrollWidgetInterpreter<T>> {
        self.get_meta_by_type(WithScroll::<T>::TYPENAME_FOR_MARGIN)
            .map(|meta| WithScrollWidgetInterpreter::new(self, meta))
    }

    pub fn get_no_editor(&self) -> Option<NoEditorInterpreter> {
        self.get_meta_by_type(NoEditorWidget::TYPENAME)
            .map(|meta| NoEditorInterpreter::new(self, meta))
            .next()
    }

    pub fn get_fuzzy_search(&self) -> Option<FuzzySearchInterpreter> {
        self.get_meta_by_type(FuzzySearchWidget::TYPENAME)
            .map(|meta| FuzzySearchInterpreter::new(self, meta))
            .next()
    }

    pub fn get_code_results_view(&self) -> Option<CodeResultsViewInterpreter> {
        self.get_meta_by_type(CodeResultsView::TYPENAME)
            .map(|meta| CodeResultsViewInterpreter::new(self, meta))
            .next()
    }
}
