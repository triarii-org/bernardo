use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::io::input_event::InputEvent;
use crate::io::output::Output;
use crate::layout::leaf_layout::LeafLayout;
use crate::primitives::sized_xy::SizedXY;
use crate::primitives::xy::XY;
use crate::widget::any_msg::AnyMsg;

// this corresponds to message to Parent.
pub type WidgetAction<W> = fn(&W) -> Option<Box<dyn AnyMsg>>;

pub type WID = usize;

pub trait Widget {
    fn id(&self) -> WID;

    fn typename(&self) -> &'static str;

    // Minimal size of the view. If the output cannot satisfy it, a replacement is drawn instead,
    // and the view cannot be focused (TODO or input will be ignored, haven't decided that yet).
    fn min_size(&self) -> XY;

    // Description of widget type
    // fn desc() -> &'static str;

    // This is guaranteed to be called before render.
    fn layout(&self, max_size: XY) -> XY;

    // If input is consumed, the output is Some(.). If you don't like it, add noop msg to your widget.
    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>>;

    // This is called when an input got consumed and internal message is created.
    // The output is a message to parent.
    // No message will NOT stop redraw.
    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>>;

    fn get_focused(&self) -> &dyn Widget;
    fn get_focused_mut(&mut self) -> &mut dyn Widget;

    fn render(&self, focused: bool, output: &mut Output);
}

pub fn get_new_widget_id() -> WID {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed) as WID
}

pub const WIDGET_NONE: usize = 0;
