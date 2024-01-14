mod display;
pub use display::MainViewDisplay;

mod display_fuzzy;
pub use display_fuzzy::DisplayItem;

mod main_view;
pub use main_view::{BufferId, DocumentIdentifier, HoverItem, MainView};

mod msg;
pub use msg::MainViewMsg;
