// This is Work in Progress. I guess the Context Bar will be more universal and moving between
// widgets (escalating). But bottom of tree is Editor, so this is where I start.

mod context_bar_item;
pub use context_bar_item::ContextBarItem;

mod msg;
pub use msg::ContextBarWidgetMsg;

mod widget;
pub use widget::ContextBarWidget;
