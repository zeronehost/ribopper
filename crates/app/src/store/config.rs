use std::str::FromStr;

use tauri_plugin_global_shortcut::{Modifiers, Shortcut};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboConfig {
  pub general: Option<RiboGeneral>,
  pub options: Option<RiboOptions>,
  pub theme: Option<RiboTheme>,
  pub hotkey: RiboHotkey,
}

impl RiboConfig {
  pub(crate) fn get_max(&self) -> anyhow::Result<Option<i64>> {
    if let Some(conf) = &self.general
      && let Some(max) = conf.max
    {
      return Ok(Some(max as i64));
    }
    Ok(None)
  }

  pub(crate) fn get_autostart(&self) -> bool {
    if let Some(conf) = &self.general {
      return conf.auto_start;
    }
    false
  }

  pub(crate) fn get_exit_confirm(&self) -> bool {
    if let Some(conf) = &self.general {
      return conf.exit_confirm;
    }
    true
  }
}

impl Default for RiboConfig {
  fn default() -> Self {
    Self {
      general: Some(RiboGeneral {
        max: None,
        auto_start: false,
        exit_confirm: true,
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
  pub exit_confirm: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiboOptions {}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboHotkey {
  pub(crate) edit: Option<RiboKey>,
  pub(crate) clear: Option<RiboKey>,
  pub(crate) prev: Option<RiboKey>,
  pub(crate) next: Option<RiboKey>,
  pub(crate) qrcode: Option<RiboKey>,
  pub(crate) pane: Option<RiboKey>,
  pub(crate) delete: Option<RiboKey>,
  pub(crate) copy: Option<RiboKey>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiboKey {
  pub(crate) alt_key: bool,
  pub(crate) ctrl_key: bool,
  pub(crate) shift_key: bool,
  pub(crate) meta_key: bool,
  pub(crate) key: String,
}

impl TryInto<Shortcut> for &RiboKey {
  type Error = anyhow::Error;
  fn try_into(self) -> Result<Shortcut, Self::Error> {
    let mut mods = Modifiers::empty();
    if self.alt_key {
      mods |= Modifiers::ALT;
    }
    if self.ctrl_key {
      mods |= Modifiers::CONTROL;
    }
    if self.shift_key {
      mods |= Modifiers::SHIFT;
    }
    if self.meta_key {
      mods |= Modifiers::SUPER;
    }
    let k = Shortcut::from_str(&self.key)?.key;
    let key = Shortcut::new(Some(mods), k);

    Ok(key)
  }
}
