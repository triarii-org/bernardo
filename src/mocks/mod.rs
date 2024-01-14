/*
All these structures are design with test in mind *only*. Meaning they are allowed to panic and
be slow to a reasonable degree.
 */

#![allow(dead_code)]

mod button_interpreter;
pub use button_interpreter::ButtonWidgetInterpreter;

mod code_results_interpreter;
pub use code_results_interpreter::CodeResultsViewInterpreter;

mod completion_interpreter;
pub use completion_interpreter::CompletionInterpreter;

mod context_bar_interpreter;
pub use context_bar_interpreter::ContextBarWidgetInterpreter;

mod editbox_interpreter;
pub use editbox_interpreter::EditWidgetInterpreter;

mod editor_interpreter;
pub use editor_interpreter::EditorInterpreter;

mod fuzzy_search_interpreter;
pub use fuzzy_search_interpreter::FuzzySearchInterpreter;

mod listview_interpreter;
pub use listview_interpreter::{ListViewInterpreter, ListViewInterpreterItem};

mod meta_frame;
pub use meta_frame::MetaOutputFrame;

mod mock_clipboard;
pub use mock_clipboard::MockClipboard;

mod mock_input;
pub use mock_input::MockInput;

mod mock_labels_provider;
pub use mock_labels_provider::MockLabelsProvider;

mod mock_navcomp_loader;
pub use mock_navcomp_loader::MockNavcompLoader;

mod mock_navcomp_promise;
pub use mock_navcomp_promise::MockNavCompPromise;

mod mock_navcomp_provider;
pub use mock_navcomp_provider::{
    MockCompletionMatcher, MockNavCompEvent, MockNavCompProvider, MockNavCompProviderPilot, MockSymbolMatcher,
};

mod mock_output;
pub use mock_output::MockOutput;

mod no_editor_interpreter;
pub use no_editor_interpreter::NoEditorInterpreter;

mod savefile_interpreter;
pub use savefile_interpreter::SaveFileInterpreter;

mod scroll_interpreter;
pub use scroll_interpreter::ScrollInterpreter;

mod treeview_interpreter;
pub use treeview_interpreter::{TreeViewInterpreter, TreeViewInterpreterItem};

mod with_scroll_interpreter;
pub use with_scroll_interpreter::WithScrollWidgetInterpreter;
