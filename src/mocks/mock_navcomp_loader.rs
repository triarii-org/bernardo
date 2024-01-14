use std::sync::{Arc, RwLock};

use crossbeam_channel::Sender;

use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::*;
use crate::*;

use crate::*;

pub struct MockNavcompLoader {
    event_sender: Sender<MockNavCompEvent>,
    completions: Arc<RwLock<Vec<MockCompletionMatcher>>>,
    symbols: Arc<RwLock<Vec<MockSymbolMatcher>>>,
}

impl MockNavcompLoader {
    pub fn new(
        event_sender: Sender<MockNavCompEvent>,
        completions: Arc<RwLock<Vec<MockCompletionMatcher>>>,
        symbols: Arc<RwLock<Vec<MockSymbolMatcher>>>,
    ) -> Self {
        MockNavcompLoader {
            event_sender,
            completions,
            symbols,
        }
    }
}

impl NavCompLoader for MockNavcompLoader {
    fn load_handler(
        &self,
        config: &ConfigRef,
        project_scope: &ProjectScope,
        navcomp_tick_sender: NavCompTickSender,
    ) -> Result<Box<dyn Handler>, HandlerLoadError> {
        debug_assert!(project_scope.handler_id.as_ref() == Some(&"rust".to_string())); // yeah I know it's shit, I have 100 compile errors

        let navcomp_op = Some(Arc::new(Box::new(MockNavCompProvider::new(
            navcomp_tick_sender,
            self.event_sender.clone(),
            self.completions.clone(),
            self.symbols.clone(),
        )) as Box<dyn NavCompProvider>));

        Ok(Box::new(RustHandler::load(config, project_scope.path.clone(), navcomp_op)?))
    }
}
