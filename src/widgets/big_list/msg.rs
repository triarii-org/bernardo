use crate::primitives::scroll_enum::ScrollEnum;

use crate::*;

#[derive(Clone, Debug)]
pub enum BigListWidgetMsg {
    Scroll(ScrollEnum),
}

impl AnyMsg for BigListWidgetMsg {}
