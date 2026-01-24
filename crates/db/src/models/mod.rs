#[cfg(feature = "action")]
mod action;
mod record;
#[cfg(feature = "action")]
mod record_action;

use crate::error::Result;
use rusqlite::Row;

pub trait FromRow: Sized {
  fn from_row(row: &Row) -> Result<Self>;
}

pub use self::record::*;
#[cfg(feature = "action")]
pub use self::{action::*, record_action::*};
