mod app;
pub use app::App;

pub mod config;
pub mod cursor;
pub mod experiments;
pub mod fs;
pub mod gladius;
pub mod io;
pub mod layout;
pub mod lsp_client;
pub mod primitives;
pub mod promise;
pub mod text;
pub mod tsw;
pub mod w7e;
pub mod widget;
pub mod widgets;

#[cfg(test)]
pub mod big_tests;

#[cfg(test)]
mod mocks;
#[cfg(test)]
pub(crate) use mocks::*;
