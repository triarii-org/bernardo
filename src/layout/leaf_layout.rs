use std::any::Any;

use log::warn;

use crate::experiments::subwidget_pointer::SubwidgetPointer;
use crate::layout::layout::{Layout, WidgetWithRect};
use crate::primitives::rect::Rect;
use crate::primitives::size_constraint::SizeConstraint;
use crate::primitives::xy::{XY, ZERO};
use crate::widget::complex_widget::ComplexWidget;
use crate::widget::widget::Widget;

pub struct LeafLayout<W: Widget> {
    widget: SubwidgetPointer<W>,
    with_border: bool,
}

impl<W: Widget> LeafLayout<W> {
    pub fn new(widget: SubwidgetPointer<W>) -> Self {
        LeafLayout { widget, with_border: false }
    }

    pub fn with_border(self) -> Self {
        LeafLayout {
            with_border: true,
            ..self
        }
    }

    fn rect(&self, output_size: XY) -> Option<Rect> {
        if self.with_border {
            if output_size > (3, 3).into() {
                Some(Rect::new(XY::new(1, 1), XY::new(output_size.x - 2, output_size.y - 2)))
            } else {
                None
            }
        } else {
            Some(Rect::new(ZERO, output_size))
        }
    }
}

impl<W: Widget> Layout<W> for LeafLayout<W> {
    fn min_size(&self, root: &W) -> XY {
        self.widget.get(root).min_size()
    }

    fn layout(&self, root: &mut W, output_size: XY) -> Vec<WidgetWithRect<W>> {
        match self.rect(output_size) {
            None => {
                warn!("too small LeafLayout to draw the view.");
                vec![]
            }
            Some(rect) => {
                let root_id = root.id();
                let widget = self.widget.get_mut(root);
                let skip = root_id == widget.id();

                if !skip {
                    widget.update_and_layout(SizeConstraint::simple(rect.size));
                }

                vec![WidgetWithRect::new(
                    self.widget.clone(),
                    rect,
                )]
            }
        }
    }

    // fn render(&self, root: &W, theme: &Theme, output: &mut dyn Output, focused: Option<WID>) {
    //     let widget = self.widget.get(root);
    //     let wid = widget.id();
    //     let focused: bool = focused == Some(wid);
    //
    //     // let sub_output = &mut SubOutput::new(output, wir.rect);
    //
    //     widget.render(theme, focused, output)
    // }
}

