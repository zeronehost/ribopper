#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Clipboard error: {0}")]
  ClipboardError(#[from] arboard::Error),
  #[error("Image error: {0}")]
  ImageError(#[from] image::ImageError),
}

pub type Result<T> = core::result::Result<T, Error>;
