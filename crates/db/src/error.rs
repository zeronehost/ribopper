#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Sqlite error: {0}")]
  Sqlite(#[from] rusqlite::Error),
  #[error("Json serialization error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Validation error: {0}")]
  Validation(String),
  #[error("Not Found {0}")]
  NotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
