/*
This describes a continous horizontal piece of buffer output, containing "text" and perhaps having a
consistent style.
 */
use crate::primitives::xy::XY;

use crate::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizontalIterItem {
    pub absolute_pos: XY,
    // Set iff style was consistent over entire item
    pub text_style: Option<TextStyle>,
    pub text: String,
}
