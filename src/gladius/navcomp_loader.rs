use crate::*;

pub trait NavCompLoader: Send + Sync + 'static {
    fn load_handler(
        &self,
        config: &ConfigRef,
        project_scope: &ProjectScope,
        navcomp_tick_sender: NavCompTickSender,
    ) -> Result<Box<dyn Handler>, HandlerLoadError>;
}
