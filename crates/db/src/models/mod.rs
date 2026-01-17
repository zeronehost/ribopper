mod record;
#[cfg(feature = "action")]
mod action;
#[cfg(feature = "action")]
mod record_action;

use crate::error::Result;
use rusqlite::Row;

pub trait FromRow: Sized {
  fn from_row(row: &Row) -> Result<Self>;
}

#[cfg(feature = "action")]
pub use self::{
  action::*,
  record_action::*,
};
pub use self::record::*;
