use crate::utils::error::Result;
use ribo_db::models::ActionWithOption;
use tauri::{
  Manager, Runtime, WebviewWindow,
  menu::{Menu, MenuItem, SubmenuBuilder},
};

pub struct Context<'a, R: Runtime> {
  menu: Menu<R>,
  app: &'a tauri::AppHandle<R>,
  content: String,
  actions: Vec<ActionWithOption>,
}

impl<'a, R: Runtime> Context<'a, R> {
  pub fn new(app: &'a tauri::AppHandle<R>, content: &str) -> Result<Self> {
    Ok(Self {
      menu: Menu::new(app)?,
      app,
      content: content.to_string(),
      actions: vec![],
    })
  }

  pub fn set_menu(&mut self, actions: Vec<ActionWithOption>) -> Result<()> {
    if actions.is_empty() {
      self.menu.append(&MenuItem::with_id(
        self.app,
        "inner_empty",
        "未查询到匹配的命令",
        false,
        None::<&str>,
      )?)?;
    }
    self.actions = actions.clone();
    for action in actions {
      let mut submenu = SubmenuBuilder::new(self.app, action.name);
      for option in action.options {
        submenu = submenu.item(&MenuItem::with_id(
          self.app,
          format!("{}_{}", action.id, option.id),
          option.name,
          true,
          None::<&str>,
        )?);
      }
      let menu = submenu.build()?;
      self.menu.append(&menu)?;
    }

    Ok(())
  }

  pub fn show(self, label: &str) -> Result<()> {
    let win = self.app.get_webview_window(label);
    if let Some(win) = win {
      win.popup_menu(&self.menu)?;
      self.on_event(win)?;
    }
    Ok(())
  }

  fn on_event(&self, win: WebviewWindow<R>) -> Result<()> {
    // 在闭包外部克隆 actions，使其拥有独立的所有权
    let actions = self.actions.clone();
    let content = self.content.clone();

    win.on_menu_event(move |_win, event| {
      // 现在闭包使用的是外部的 actions，而不是 self.actions
      let parts: Vec<&str> = event.id.as_ref().split('_').collect();
      // 使用 match 处理不同长度的切片
      let (action_id, option_id) = match parts.as_slice() {
        [action_id, option_id] => (*action_id, *option_id),
        _ => {
          log::error!(
            "Invalid ID format: expected 'actionId_optionId', got {:?}",
            event.id.as_ref()
          );
          return; // 格式不对，直接返回
        }
      };
      let action_id = match action_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
          log::error!("Failed to parse action_id: {}", action_id);
          return;
        }
      };
      let option_id = match option_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => {
          log::error!("Failed to parse option_id: {}", option_id);
          return;
        }
      };
      if let Some(action) = actions.iter().find(|a| a.id == action_id) {
        if let Some(option) = action.options.iter().find(|o| o.id == option_id) {
          let cmd = option.command.replace("<%s>", &content);
          // 注意：这里执行命令的方式可能也需要根据平台调整
          log::info!("Executing command: {}", cmd);
          #[cfg(target_os = "windows")]
          {
            if let Err(e) = std::process::Command::new("powershell")
              .arg(&format!("-e {cmd}"))
              .spawn()
            {
              log::error!("Failed to execute command: {}", e);
            }
          }
          #[cfg(target_os = "linux")]
          {
            if let Err(e) = std::process::Command::new("xfce4-terminal")
              .arg(&format!("-e {cmd}"))
              .spawn()
            {
              log::error!("Failed to execute command: {}", e);
            }
          }
          #[cfg(target_os = "macos")]
          {
            if let Err(e) = std::process::Command::new("open")
              .arg("-a")
              .arg("Terminal")
              .arg(&format!("-e {cmd}"))
              .spawn()
            {
              log::error!("Failed to execute command: {}", e);
            }
          }
        } else {
          log::error!(
            "Option with id {} not found in action {}",
            option_id,
            action_id
          );
        }
      } else {
        log::error!("Action with id {} not found", action_id);
      }
    });
    Ok(())
  }
}
