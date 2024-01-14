pub mod logger_setup;
pub mod navcomp_loader;
pub mod paradigm;
pub mod providers;
pub mod real_navcomp_loader;
pub mod run_gladius;

#[cfg(test)]
mod full_setup;
#[cfg(test)]
pub(crate) use full_setup::FullSetup;

#[cfg(test)]
mod big_tests;
