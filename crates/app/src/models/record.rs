use std::path::PathBuf;

use chrono::{DateTime, Local};

pub(crate) enum Record {
  Text {
    id: u64,
    content: String,
    data: Vec<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
  Image {
    id: u64,
    content: Vec<u8>,
    data: Vec<u8>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
  Files {
    id: u64,
    content: Vec<PathBuf>,
    data: Vec<PathBuf>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
}
