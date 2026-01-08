mod action;
mod record;
mod record_action;

use crate::error::Result;
use rusqlite::Row;

pub trait FromRow: Sized {
  fn from_row(row: &Row) -> Result<Self>;
}

pub use self::{
  action::*,
  record::*,
  record_action::*,
};
