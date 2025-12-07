#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Vec<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: Vec<RiboHotkey>,
}

impl Default for RiboConfig {
  fn default() -> Self {
    Self {
      general: None,
      options: vec![RiboOptions::Text(TypeOption {
        editable: true,
        deletable: true,
        scannable: true,
        starable: true,
      })],
      theme: Some(RiboTheme::Light),
      hotkey: vec![],
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
pub struct RiboGeneral {
  pub max: Option<usize>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboOptions {
  Text(TypeOption),
  Image(TypeOption),
  File(TypeOption),
  Dir(TypeOption),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeOption {
  pub editable: bool,
  pub deletable: bool,
  pub scannable: bool,
  pub starable: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboHotkey {
  Edit(Vec<String>),
}
