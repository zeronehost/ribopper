#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Clipboard error: {0}")]
  ClipboardError(#[from] arboard::Error),
  #[cfg(feature = "image")]
  #[error("Image error: {0}")]
  ImageError(#[from] image::ImageError),
  #[cfg(feature = "image")]
  #[error("Image not exists")]
  ImageNotExists,
}

pub type Result<T> = core::result::Result<T, Error>;
