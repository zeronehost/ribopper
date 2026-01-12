pub mod config;
pub mod record;
pub mod window;
pub mod common;
#[cfg(feature = "action")]
pub mod action;

type CommandResult<T> = Result<T, String>;
