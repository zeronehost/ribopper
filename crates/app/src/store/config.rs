#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Option<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: Vec<RiboHotkey>,
}

impl Default for RiboConfig {
  fn default() -> Self {
    Self {
      general: Some(RiboGeneral {
        max: None,
        auto_start: false,
        duration: 500,
        options: RiboTypeOptions::default(),
      }),
      options: None,
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
#[serde(rename_all = "camelCase")]
pub struct RiboGeneral {
  pub max: Option<usize>,
  pub auto_start: bool,
  pub duration: u64,
  pub options: RiboTypeOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboOptions {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboTypeOptions {
  pub text: TypeOption,
  pub image: TypeOption,
  pub file: TypeOption,
  pub dir: TypeOption,
}

impl Default for RiboTypeOptions {
  fn default() -> Self {
    Self {
      text: TypeOption::default(),
      image: TypeOption::default(),
      file: TypeOption::default(),
      dir: TypeOption::default(),
    }
  }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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
