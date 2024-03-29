use std::sync::Arc;

use log::{debug, error};

use crate::*;

/*
This is a single point of entry to loading LanguageHandlers, to be used by both workspace generator
    and deserializer
 */
pub fn handler_factory(
    config: &ConfigRef,
    handler_id: &str,
    ff: SPath,
    navcomp_tick_sender: NavCompTickSender,
) -> Result<Box<dyn Handler>, HandlerLoadError> {
    debug!("attempting to load handler {} for {:?}", handler_id, ff.absolute_path());
    match handler_id {
        "rust" => {
            //So handler can "partially work", meaning for instance that running/debugging works, but LSP does
            // not. TODO move lsp_path to workspace?
            let lsp_path = config.global.get_rust_lsp_path().ok_or(HandlerLoadError::LspNotFound)?;
            let workspace_root = ff.absolute_path();
            let mut navcomp_op: Option<NavCompRef> = None;
            if let Some(navcomp_lsp) = NavCompProviderLsp::new(lsp_path, workspace_root, navcomp_tick_sender) {
                navcomp_op = Some(Arc::new(Box::new(navcomp_lsp)));
            } else {
                error!("LspWrapper construction failed.")
            }

            match RustHandler::load(config, ff, navcomp_op) {
                Ok(o) => Ok(Box::new(o)),
                Err(e) => Err(e),
            }
        }
        _ => Err(HandlerLoadError::HandlerNotFound),
    }
}
