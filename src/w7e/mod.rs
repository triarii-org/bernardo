// This module is "workspace"

mod buffer_state_shared_ref;
pub use buffer_state_shared_ref::{BufferR, BufferRW, BufferSharedRef};

mod handler;
pub use handler::{Handler, NavCompRef};

mod handler_factory;
pub use handler_factory::handler_factory;

mod handler_load_error;
pub use handler_load_error::HandlerLoadError;

mod inspector;
pub use inspector::{inspect_workspace, InspectError, LangInspector};

mod navcomp_group;
pub use navcomp_group::{NavCompGroup, NavCompGroupRef, NavCompTick, NavCompTickRecv, NavCompTickSender};

mod navcomp_provider;
pub use navcomp_provider::{
    Completion, CompletionAction, CompletionsPromise, FormattingPromise, NavCompProvider, NavCompSymbol, NavCompSymbolContextActions,
    StupidSubstituteMessage, SymbolContextActionsPromise, SymbolPromise, SymbolType, SymbolUsage, SymbolUsagesPromise,
};

mod navcomp_provider_lsp;
pub use navcomp_provider_lsp::{LspError, NavCompProviderLsp};

mod project_scope;
pub use project_scope::{ProjectLoadError, ProjectScope, SerializableProjectScope};

mod rust;
pub use rust::*;

mod suggestions_provider;
pub use suggestions_provider::SuggestionProvider;

mod workspace;
pub use workspace::{ScopeLoadErrors, Scopes, SerializableWorkspace, Workspace, WorkspaceLoadError, WORKSPACE_FILE_NAME};
