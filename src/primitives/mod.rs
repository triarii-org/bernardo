mod macros;
mod rope_buffer_state;

mod arrow;
pub use arrow::Arrow;

mod border;
pub use border::{BorderStyle, SINGLE_BORDER_STYLE};

mod color;
pub use color::{Color, BLACK, WHITE};

mod helpers;
pub use helpers::{copy_first_n_columns, copy_last_n_columns, fill_output, get_next_filename};

mod rect;
pub use rect::{CornersIterator, Rect};

mod sized_xy;
pub use sized_xy::SizedXY;

// TODO(XXX) remove
mod styled_string;

mod xy;
pub use xy::{NeighboursIterator, XY};

mod scroll;
pub use scroll::{Scroll, ScrollDirection};

mod common_edit_msgs;
pub use common_edit_msgs::{CommonEditMsg, _apply_cem, cme_to_direction, key_to_edit_msg};

mod common_query;
pub use common_query::CommonQuery;

mod is_default;
pub use is_default::IsDefault;

// TODO(XXX): remove
mod provider;

mod search_pattern;
pub use search_pattern::SearchPattern;

mod stupid_cursor;
pub use stupid_cursor::StupidCursor;

mod tmtheme;
pub use tmtheme::TmTheme;

mod has_invariant;
pub use has_invariant::HasInvariant;

mod printable;
pub use printable::Printable;

mod styled_printable;
pub use styled_printable::{StyleBorrowedPrintable, StyleWrappedPrintable, StyledPrintable};

mod scroll_enum;
pub use scroll_enum::ScrollEnum;

#[cfg(test)]
mod tests;
