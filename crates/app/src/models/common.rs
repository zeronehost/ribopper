#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct AppInfo {
  pub(crate) name: String,
  pub(crate) version: String,
  pub(crate) description: String,
  pub(crate) authors: String,
  pub(crate) license: String,
  pub(crate) website: String,
}