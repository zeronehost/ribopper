use std::path::{Path, PathBuf};

use crate::error::Result;

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
  pub(crate) fn get_image(&mut self) -> Result<Vec<u8>> {
    let image = self.0.get_image()?;
    Ok(image.into_owned_bytes().to_vec())
  }
  pub(crate) fn set_image(&mut self, data: &[u8]) -> Result<()> {
    let image_data = image::load_from_memory(data)?;
    let width = image_data.width() as usize;
    let height = image_data.height() as usize;
    let bytes = image_data.into_bytes().into();
    let data = arboard::ImageData {
      width,
      height,
      bytes,
    };
    self.0.set_image(data).map_err(Into::into)
  }

  pub(crate) fn get_files(&mut self) -> Result<Vec<PathBuf>> {
    let clipboard_get = self.0.get();
    clipboard_get.file_list().map_err(Into::into)
  }
  pub(crate) fn set_files(&mut self, files: &[impl AsRef<Path>]) -> Result<()> {
    let clipboard_set = self.0.set();
    clipboard_set.file_list(files).map_err(Into::into)
  }
}
