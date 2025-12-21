#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Option<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: Vec<RiboHotkey>,
  pub schema: String,
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboOptions {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiboHotkey {
  Edit(Vec<String>),
}
