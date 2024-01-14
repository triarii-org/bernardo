use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::unpack_or;



use crate::*;

pub struct NoEditorWidget {
    wid: WID,
    text_pos: XY,

    last_size: Option<XY>,
}

impl NoEditorWidget {
    pub const TYPENAME: &'static str = "no_editor_widget";
    pub const NO_EDIT_TEXT: &'static str = "no editor loaded.";
}

impl Default for NoEditorWidget {
    fn default() -> Self {
        NoEditorWidget {
            wid: get_new_widget_id(),
            text_pos: XY::ZERO,
            last_size: None,
        }
    }
}

impl Widget for NoEditorWidget {
    fn id(&self) -> WID {
        self.wid
    }
    fn static_typename() -> &'static str
    where
        Self: Sized,
    {
        Self::TYPENAME
    }
    fn typename(&self) -> &'static str {
        Self::TYPENAME
    }

    fn full_size(&self) -> XY {
        XY::new(Self::NO_EDIT_TEXT.len() as u16, 3)
    }

    fn layout(&mut self, screenspace: Screenspace) {
        let size = screenspace.output_size();

        let mut x = 0;
        if size.x >= Self::NO_EDIT_TEXT.len() as u16 {
            x = (size.x - Self::NO_EDIT_TEXT.len() as u16) / 2;
        };

        let y = size.y / 2;

        self.text_pos = XY::new(x, y);
        self.last_size = Some(size);
    }

    fn on_input(&self, _input_event: InputEvent) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn update(&mut self, _msg: Box<dyn AnyMsg>) -> Option<Box<dyn AnyMsg>> {
        None
    }

    fn render(&self, theme: &Theme, focused: bool, output: &mut dyn Output) {
        #[cfg(test)]
        {
            let size = unpack_or!(self.last_size, (), "render before layout");
            output.emit_metadata(Metadata {
                id: self.wid,
                typename: self.typename().to_string(),
                rect: Rect::from_zero(size),
                focused,
            });
        }

        // fill_background(theme.default_background(focused), output);

        output.print_at(self.text_pos, theme.default_text(focused), Self::NO_EDIT_TEXT);
    }
}
