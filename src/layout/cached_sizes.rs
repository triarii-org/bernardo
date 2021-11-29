use std::borrow::{Borrow, BorrowMut};

use crate::experiments::focus_group::FocusGroup;
use crate::experiments::from_geometry::from_wirs;
use crate::layout::layout::WidgetIdRect;
use crate::primitives::xy::XY;

//TODO: more advanced option would store references to widgets instead of their WIDs.
// I'll consider that in a next step.

#[derive(Debug)]
pub struct DisplayState {
    pub for_size: XY,
    pub widget_sizes: Vec<WidgetIdRect>,
    pub focus_group: Box<dyn FocusGroup>,
}

impl DisplayState {
    // pub fn new2(for_size: XY, widget_sizes: Vec<WidgetIdRect>) -> Self {
    //
    //
    //     DisplayState::new(for_size, widget_sizes)
    // }

    pub fn focus_group_mut(&mut self) -> &mut dyn FocusGroup {
        self.focus_group.as_mut()
    }

    pub fn focus_group(&self) -> &dyn FocusGroup {
        self.focus_group.borrow()
    }

    pub fn new(for_size: XY, widget_sizes: Vec<WidgetIdRect>) -> Self {
        let focus_group = from_wirs(&widget_sizes, Some(for_size));
        DisplayState {
            for_size,
            widget_sizes,
            focus_group: Box::new(focus_group),
        }
    }
}