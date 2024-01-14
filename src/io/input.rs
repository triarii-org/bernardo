use crate::*;

pub trait Input {
    fn source(&self) -> &InputSource;
}
