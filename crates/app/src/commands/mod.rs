// pub mod db;
// pub mod func;
// pub mod store;

pub mod record;
pub mod window;
pub mod config;
pub mod target;

type CommandResult<T> = Result<T, String>;