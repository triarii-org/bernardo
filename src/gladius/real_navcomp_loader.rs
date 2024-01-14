use log::warn;

use crate::gladius::navcomp_loader::NavCompLoader;
use crate::*;

pub struct RealNavCompLoader {}

impl RealNavCompLoader {
    pub fn new() -> Self {
        RealNavCompLoader {}
    }
}

impl NavCompLoader for RealNavCompLoader {
    /*
    Config is required to "know" where the LSP servers are. We will provide reasonable defaults,
    but option to override is essential.
    It just occurred to me: I might want to have overrides local to workspace.
     */
    fn load_handler(
        &self,
        config: &ConfigRef,
        project_scope: &ProjectScope,
        navcomp_tick_sender: NavCompTickSender,
    ) -> Result<Box<dyn Handler>, HandlerLoadError> {
        match &project_scope.handler_id {
            None => {
                warn!(
                    "project scope [{:?}] with no handler - what the point?",
                    project_scope.path.relative_path()
                );
                Err(HandlerLoadError::NoHandlerId)
            }
            Some(handler_id) => Ok(handler_factory(
                config,
                &handler_id,
                project_scope.path.clone(),
                navcomp_tick_sender.clone(),
            )?),
        }
    }
}
