mod config;
pub use config::{Config, ConfigRef};

mod global_editor_options;
pub use global_editor_options::GlobalEditorOptions;

mod load_error;
pub use load_error::ConfigLoadError;

mod save_error;
pub use save_error::SaveError;

mod theme;
pub use theme::{CursorsSettings, Theme, UiTheme};
