pub mod config;
pub mod record;
pub mod window;
pub mod action;
pub mod common;

type CommandResult<T> = Result<T, String>;
