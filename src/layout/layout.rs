use std::iter::Iterator;

use crate::experiments::focus_group::FocusUpdate;
use crate::io::output::Output;
use crate::primitives::rect::Rect;
use crate::primitives::theme::Theme;
use crate::primitives::xy::XY;
use crate::widget::widget::{WID, Widget};

pub type WidgetGetter<T: Widget> = Box<dyn Fn(&'_ T) -> &'_ dyn Widget>;
pub type WidgetGetterMut<T: Widget> = Box<dyn Fn(&'_ mut T) -> &'_ mut dyn Widget>;


#[derive(Clone, Copy, Debug)]
pub struct WidgetIdRect {
    pub wid: WID,
    pub rect: Rect,
}

pub trait Layout {
    fn is_leaf(&self) -> bool {
        false
    }

    fn min_size(&self) -> XY;

    /*
    This only calculates the rects under current constraints. The widgets themselves should
    receive information about their new sizes before render.
     */
    fn calc_sizes(&mut self, output_size: XY) -> Vec<WidgetIdRect>;

    fn draw_border(&self, theme: &Theme, focused: bool, output: &mut Output);
}
