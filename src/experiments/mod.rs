mod array_streaming_iterator;
pub use array_streaming_iterator::ArrayStreamingIt;

mod buffer_register;
pub use buffer_register::{BufferRegister, BufferRegisterRef, OpenResult};

mod clipboard;
pub use clipboard::{get_me_fake_clipboard, get_me_some_clipboard, Clipboard, ClipboardRef};

mod filename_to_language;
pub use filename_to_language::filename_to_language;

mod focus_group;
pub use focus_group::{FocusGraph, FocusGraphNode, FocusUpdate};

mod from_geometry;
pub use from_geometry::from_geometry;

mod grapheme_lines_streaming_iterator;
pub use grapheme_lines_streaming_iterator::GraphemeLinesStreamingIterator;

// TODO(XXX): remove
mod path_prefixes;

mod pretty_ron;
pub use pretty_ron::ToPrettyRonString;

mod regex_search;
pub use regex_search::{regex_find, FindError, RegexMatches};

mod screen_shot;
pub use screen_shot::screenshot;

mod screenspace;
pub use screenspace::Screenspace;

mod subwidget_pointer;
pub use subwidget_pointer::{Getter, GetterMut, GetterOp, GetterOpMut, SubwidgetPointer, SubwidgetPointerOp};
