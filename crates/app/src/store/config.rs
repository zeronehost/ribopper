#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Option<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: Vec<RiboHotkey>,
  pub schema: String,
}

impl Default for RiboConfig {
  fn default() -> Self {
    Self {
      general: Some(RiboGeneral {
        max: None,
        auto_start: false,
        duration: 500,
      }),
      options: None,
      theme: Some(RiboTheme::Light),
      hotkey: vec![],
      schema: if cfg!(target_os = "windows") {
        "https://ribopper.".to_string()
      } else {
        "ribopper://".to_string()
      },
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboTheme {
  Light,
  Dark,
  Auto,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboGeneral {
  pub max: Option<usize>,
  pub auto_start: bool,
  pub duration: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboOptions {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboHotkey {
  Edit(Vec<String>),
}
