use crate::*;

pub struct ActionTrigger<W: Widget> {
    title: String,
    trigger: Box<dyn FnOnce(&W) -> Option<Box<dyn AnyMsg>>>,
}

impl<W: Widget> ActionTrigger<W> {
    pub fn new(title: String, trigger: Box<dyn FnOnce(&W) -> Option<Box<dyn AnyMsg>>>) -> Self {
        ActionTrigger { title, trigger }
    }
}
