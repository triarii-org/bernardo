mod buffer_state;
pub use buffer_state::{BufferState, BufferStateStreamingIterator, BufferType, SetFilePathResult};

mod buffer_state_fuzz;

mod text_buffer;
pub use text_buffer::{LinesIter, TextBuffer};

mod contents_and_cursors;
pub use contents_and_cursors::ContentsAndCursors;

#[cfg(test)]
mod rope_tests;

#[cfg(test)]
mod buffer_state_test;
