use std::fmt::{Debug, Formatter};
use std::io::Error;

use crate::*;

pub struct LocalMockOutput {
    pub size: XY,
    pub visible_rect: Rect,
}

impl SizedXY for LocalMockOutput {
    fn size(&self) -> XY {
        self.size
    }
}

impl Debug for LocalMockOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalMockOutput")
    }
}

impl Output for LocalMockOutput {
    fn print_at(&mut self, _pos: XY, _style: TextStyle, _text: &str) {
        unimplemented!()
    }

    fn clear(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn visible_rect(&self) -> Rect {
        self.visible_rect
    }

    fn emit_metadata(&mut self, _meta: Metadata) {
        unimplemented!()
    }
}
