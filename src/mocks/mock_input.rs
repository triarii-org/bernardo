use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::*;
use crate::*;
use crate::*;

pub struct MockInput {
    receiver: Receiver<InputEvent>,
}

impl Input for MockInput {
    fn source(&self) -> &InputSource {
        &self.receiver
    }
}

impl MockInput {
    pub fn new() -> (MockInput, Sender<InputEvent>) {
        let (sender, receiver) = unbounded::<InputEvent>();

        (MockInput { receiver }, sender)
    }
}
