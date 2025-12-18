// pub mod db;
// pub mod func;
// pub mod store;

pub mod config;
pub mod record;
pub mod target;
pub mod window;

type CommandResult<T> = Result<T, String>;
