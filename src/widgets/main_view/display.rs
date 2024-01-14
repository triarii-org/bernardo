use crate::*;

pub enum MainViewDisplay {
    Editor(EditorView),
    ResultsView(CodeResultsView),
}

impl MainViewDisplay {
    pub fn get_widget(&self) -> &dyn Widget {
        match self {
            MainViewDisplay::Editor(e) => e,
            MainViewDisplay::ResultsView(r) => r,
        }
    }

    pub fn get_widget_mut(&mut self) -> &mut dyn Widget {
        match self {
            MainViewDisplay::Editor(e) => e,
            MainViewDisplay::ResultsView(r) => r,
        }
    }
}
