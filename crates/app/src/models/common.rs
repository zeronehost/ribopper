#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppInfo {
  pub(crate) name: String,
  pub(crate) version: String,
  pub(crate) description: String,
  pub(crate) authors: String,
  pub(crate) license: String,
  pub(crate) website: String,
  pub(crate) features: Features,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Features {
  pub(crate) action: bool,
  pub(crate) image: bool,
}

impl Default for Features {
  fn default() -> Self {
    Self {
      action: cfg!(feature = "action"),
      image: cfg!(feature = "image"),
    }
  }
}
