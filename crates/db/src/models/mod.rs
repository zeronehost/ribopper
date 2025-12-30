mod command;
mod record;
mod record_target;
mod target;

use crate::error::Result;
use rusqlite::Row;

pub trait FromRow: Sized {
  fn from_row(row: &Row) -> Result<Self>;
}

pub use self::{command::*, record::*, record_target::*, target::*};
