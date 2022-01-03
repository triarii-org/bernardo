use crate::{AnyMsg, InputEvent, Output, Theme, Widget};
use crate::io::style::TextStyle;
use crate::primitives::size_constraint::SizeConstraint;
use crate::primitives::xy::{XY, ZERO};
use crate::widget::widget::{get_new_widget_id, WID};

const NO_EDIT_TEXT: &'static str = "no editor loaded.";

pub struct NoEditorWidget {
    wid: WID,
    text_pos: XY,
}

impl NoEditorWidget {
    pub fn new() -> Self {
        NoEditorWidget {
            wid: get_new_widget_id(),
            text_pos: ZERO,
        }
    }
}

impl Widget for NoEditorWidget {
    fn id(&self) -> WID {
        self.wid
    }

    fn typename(&self) -> &'static str {
        "no_editor_widget"
    }

    fn min_size(&self) -> XY {
        XY::new(5, 3)
    }

    fn layout(&mut self, sc: SizeConstraint) -> XY {
        let x = (sc.hint().x + NO_EDIT_TEXT.len() as u16) / 2;
        let y = sc.hint().y / 2;

        self.text_pos = XY::new(x, y);

        sc.hint()
    }

    fn on_input(&self, input_event: InputEvent) -> Option<Box<dyn AnyMsg>> { None }

    fn update(&mut self, msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        fill_background(theme.default_background(focused), output);

        output.print_at(self.text_pos,
                        theme.default_text(focused),
                        NO_EDIT_TEXT,
        );
    }
}