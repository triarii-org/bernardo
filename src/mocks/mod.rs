/*
All these structures are design with test in mind *only*. Meaning they are allowed to panic and
be slow to a reasonable degree.
 */

#![allow(dead_code)]

mod button_interpreter;
pub(crate) use button_interpreter::ButtonWidgetInterpreter;

mod code_results_interpreter;
pub(crate) use code_results_interpreter::CodeResultsViewInterpreter;

mod completion_interpreter;
pub(crate) use completion_interpreter::CompletionInterpreter;

mod context_bar_interpreter;
pub(crate) use context_bar_interpreter::ContextBarWidgetInterpreter;

mod editbox_interpreter;
pub(crate) use editbox_interpreter::EditWidgetInterpreter;

mod editor_interpreter;
pub(crate) use editor_interpreter::EditorInterpreter;

mod fuzzy_search_interpreter;
pub(crate) use fuzzy_search_interpreter::FuzzySearchInterpreter;

mod listview_interpreter;
pub(crate) use listview_interpreter::{ListViewInterpreter, ListViewInterpreterItem};

mod meta_frame;
pub(crate) use meta_frame::MetaOutputFrame;

mod mock_clipboard;
pub(crate) use mock_clipboard::MockClipboard;

mod mock_input;
pub(crate) use mock_input::MockInput;

mod mock_labels_provider;
pub(crate) use mock_labels_provider::MockLabelsProvider;

mod mock_navcomp_loader;
pub(crate) use mock_navcomp_loader::MockNavcompLoader;

mod mock_navcomp_promise;
pub(crate) use mock_navcomp_promise::MockNavCompPromise;

mod mock_navcomp_provider;
pub(crate) use mock_navcomp_provider::{
    MockCompletionMatcher, MockNavCompEvent, MockNavCompProvider, MockNavCompProviderPilot, MockSymbolMatcher,
};

mod mock_output;
pub(crate) use mock_output::MockOutput;

mod no_editor_interpreter;
pub(crate) use no_editor_interpreter::NoEditorInterpreter;

mod savefile_interpreter;
pub(crate) use savefile_interpreter::SaveFileInterpreter;

mod scroll_interpreter;
pub(crate) use scroll_interpreter::ScrollInterpreter;

mod treeview_interpreter;
pub(crate) use treeview_interpreter::{TreeViewInterpreter, TreeViewInterpreterItem};

mod with_scroll_interpreter;
pub(crate) use with_scroll_interpreter::WithScrollWidgetInterpreter;
