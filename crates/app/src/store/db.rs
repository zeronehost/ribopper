use std::{path::Path, sync::Mutex};

use ribo_db::Database;
use crate::utils::error::Result;

pub struct Db(pub Mutex<Database>);

impl Db {
  pub fn new<P: AsRef<Path>>(path: P, key: Option<&str>) -> Result<Self> {
    let db = Database::new(path, key)?;
    db.init()?;

    Ok(Self(Mutex::new(db)))
  }
}
