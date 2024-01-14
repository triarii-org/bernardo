mod cursor;
pub use cursor::{BackwardWordDeterminant, Cursor, CursorStatus, ForwardWordDeterminant, Selection};

mod cursor_set;
pub use cursor_set::CursorSet;

mod cursor_set_fuzz;

mod cursor_set_rect;
pub use cursor_set_rect::cursor_set_to_rect;

mod constants;
pub use constants::{NEWLINE_WIDTH, ZERO_CURSOR};

#[cfg(test)]
mod tests;
#[cfg(test)]
pub use tests::helpers::*;
