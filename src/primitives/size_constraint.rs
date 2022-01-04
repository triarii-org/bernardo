use std::fmt::{Debug, Display, Formatter};

use crate::primitives::rect::Rect;
use crate::primitives::xy::{XY, ZERO};

//TODO find a shorter name

/*
Contracts:
x >= hint.lower_right.x || x == None,
y >= hint.lower_right.y || y == None,
None means "no limit"
 */
#[derive(Copy, Clone, Debug)]
pub struct SizeConstraint {
    x: Option<u16>,
    y: Option<u16>,

    // this corresponds to actual screen pos and size (visible part).
    rect: Rect,
}

impl Display for SizeConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self.x {
            Some(x) => format!("{} (hint {})", x, self.rect.size.x),
            None => format!("unlimited (hint {})", self.rect.size.x),
        };

        let y = match self.y {
            Some(y) => format!("{} (hint {})", y, self.rect.size.y),
            None => format!("unlimited (hint {})", self.rect.size.y)
        };

        write!(f, "sc:[{}, {}][off {}]", x, y, self.rect.pos)
    }
}

impl SizeConstraint {
    pub fn new(x: Option<u16>, y: Option<u16>, rect: Rect) -> Self {
        SizeConstraint {
            x,
            y,
            rect,
        }
    }

    pub fn simple(xy: XY) -> Self {
        SizeConstraint {
            x: Some(xy.x),
            y: Some(xy.y),
            rect: Rect::new(ZERO, xy),
        }
    }

    pub fn x(&self) -> Option<u16> {
        self.x
    }

    pub fn y(&self) -> Option<u16> {
        self.y
    }

    // This corresponds to VISIBLE PART of output. It is used for two things:
    // - drawing optimisation
    // - layouting views that want to "fill" the visible part.
    pub fn hint(&self) -> &Rect {
        &self.rect
    }

    pub fn bigger_equal_than(&self, xy: XY) -> bool {
        self.x.map(|x| x >= xy.x).unwrap_or(true) &&
            self.y.map(|y| y >= xy.y).unwrap_or(true)
    }

    pub fn strictly_bigger_than(&self, xy: XY) -> bool {
        self.x.map(|x| x > xy.x).unwrap_or(true) &&
            self.y.map(|y| y > xy.y).unwrap_or(true)
    }
}