//! Anything in here can and should be moved to a separate Gladius crate.

mod args;
pub use args::Args;

pub mod constants;
pub use constants::{CONFIG_FILE_NAME, PROGRAM_NAME};

mod load_config;
pub use load_config::load_config;

mod real_navcomp_loader;
pub use real_navcomp_loader::RealNavCompLoader;

mod run_gladius;
pub use run_gladius::run_gladius;

mod logger_setup;
pub use logger_setup::logger_setup;

#[cfg(test)]
mod full_setup;
#[cfg(test)]
pub(crate) use full_setup::FullSetup;

#[cfg(test)]
mod big_tests;
