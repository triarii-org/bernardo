/*
This widget will be full-page (or editor-size) search results, that can either point to:
    - places in code
    or
    - references to a method
 */

mod code_results_msg;
pub use code_results_msg::CodeResultsMsg;

mod code_results_provider;
pub use code_results_provider::{CodeResultsProvider, PollResult};

mod code_results_widget;
pub use code_results_widget::CodeResultsView;

mod promise_provider;
pub use promise_provider::WrappedSymbolUsagesPromise;
