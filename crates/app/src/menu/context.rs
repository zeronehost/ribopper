use ribo_db::models::ActionWithOption;
use tauri::{
  Manager, Runtime,
  menu::{Menu, MenuItem, SubmenuBuilder},
};

pub struct Context<'a, R: Runtime> {
  menu: Menu<R>,
  app: &'a tauri::AppHandle<R>,
}

impl<'a, R: Runtime> Context<'a, R> {
  pub fn new(app: &'a tauri::AppHandle<R>) -> anyhow::Result<Self> {
    Ok(Self {
      menu: Menu::new(app)?,
      app,
    })
  }

  pub fn set_menu(&mut self, actions: Vec<ActionWithOption>) -> anyhow::Result<()> {
    if actions.is_empty() {
      self.menu.append(&MenuItem::with_id(
        self.app,
        "inner_empty",
        "未查询到匹配的命令",
        false,
        None::<&str>,
      )?)?;
    }
    for action in actions {
      let mut submenu = SubmenuBuilder::new(self.app, action.name);
      for option in action.options {
        submenu = submenu.item(&MenuItem::with_id(
          self.app,
          option.id,
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

  pub fn show(self, label: &str) -> anyhow::Result<()> {
    let win = self.app.get_webview_window(label);
    if let Some(win) = win {
      win.popup_menu(&self.menu)?;
    }
    Ok(())
  }
}
