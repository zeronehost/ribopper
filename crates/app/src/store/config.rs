#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Option<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: RiboHotkey,
}

impl RiboConfig {
  pub(crate) fn get_max(&self) -> anyhow::Result<Option<i64>> {
    if let Some(conf) = &self.general {
      if let Some(max) = conf.max {
        return Ok(Some(max as i64));
      }
    }
    Ok(None)
  }

  pub(crate) fn get_autostart(&self) -> bool {
    if let Some(conf) = &self.general {
      return conf.auto_start;
    }
    false
  }
}

impl Default for RiboConfig {
  fn default() -> Self {
    Self {
      general: Some(RiboGeneral {
        max: None,
        auto_start: false,
      }),
      options: None,
      theme: Some(RiboTheme::default()),
      hotkey: RiboHotkey::default(),
    }
  }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboTheme {
  #[default]
  Light,
  Dark,
  Auto,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboGeneral {
  pub max: Option<usize>,
  pub auto_start: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboOptions {}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboHotkey {
  edit: Option<RiboKey>,
  clear: Option<RiboKey>,
  prev: Option<RiboKey>,
  next: Option<RiboKey>,
  qrcode: Option<RiboKey>,
  pane: Option<RiboKey>,
  delete: Option<RiboKey>,
  copy: Option<RiboKey>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboKey {
  alt_key: bool,
  ctrl_key: bool,
  shift_key: bool,
  meta_key: bool,
  key: String,
}