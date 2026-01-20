use std::path::{Path, PathBuf};

use crate::error::Result;
#[cfg(feature = "image")]
use crate::models::Image;

pub struct Clipboard(pub(crate) arboard::Clipboard);

impl From<arboard::Clipboard> for Clipboard {
  fn from(value: arboard::Clipboard) -> Self {
    Self(value)
  }
}

impl Clipboard {
  pub(crate) fn new() -> Result<Self> {
    let clipboard = arboard::Clipboard::new()?;
    Ok(Self(clipboard))
  }
  pub(crate) fn get_text(&mut self) -> Result<String> {
    self.0.get_text().map_err(Into::into)
  }
  pub(crate) fn set_text(&mut self, text: &str) -> Result<()> {
    self.0.set_text(text).map_err(Into::into)
  }

  #[cfg(feature = "image")]
  pub(crate) fn get_image(&mut self) -> Result<Image> {

    let image = self.0.get_image()?;
    Ok(Image {
      width: image.width as u32,
      height: image.height as u32,
      data: image.bytes.to_vec(),
    })
  }

  #[cfg(feature = "image")]
  pub(crate) fn set_image(&mut self, data: Image) -> Result<()> {
    let data = arboard::ImageData {
      width: data.width as usize,
      height: data.height as usize,
      bytes: data.data.into(),
    };
    self.0.set_image(data).map_err(Into::into)
  }

  #[cfg(feature = "file")]
  pub(crate) fn get_files(&mut self) -> Result<Vec<PathBuf>> {
    let clipboard_get = self.0.get();
    clipboard_get.file_list().map_err(Into::into)
  }
  #[cfg(feature = "file")]
  pub(crate) fn set_files(&mut self, files: &[impl AsRef<Path>]) -> Result<()> {
    let clipboard_set = self.0.set();
    clipboard_set.file_list(files).map_err(Into::into)
  }
}
