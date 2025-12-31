pub mod config;
pub mod record;
pub mod target;
pub mod window;
pub mod command;

type CommandResult<T> = Result<T, String>;
