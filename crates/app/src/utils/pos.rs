use super::error::Result;
use tauri::{PhysicalPosition, Runtime};

pub fn set_tray_window_pos<R: Runtime>(
  app: &tauri::AppHandle<R>,
  window: &tauri::WebviewWindow<R>,
) -> Result<()> {
  log::info!("Setting tray window position");
  let pos = app.cursor_position()?;
  log::info!("Cursor position: {:?}", pos);
  let monitor = window
    .current_monitor()?
    .unwrap_or(app.monitor_from_point(pos.x, pos.y)?.unwrap());
  log::info!("Monitor: {:?}", monitor.name());
  let scale = window.scale_factor()?;
  log::info!("Scale: {}", scale);
  let baseline_h_physical = (600. * scale).round() as u32;
  let baseline_w_physical = (360. * scale).round() as u32;
  log::info!("Baseline size: {baseline_w_physical}x{baseline_h_physical}");
  let monitor_size = monitor.work_area().size;
  log::info!("Monitor size: {:?}", monitor_size);
  // 计算当前窗口显示位置
  let is_left = pos.x < (monitor_size.width / 2) as f64;
  let is_top = pos.y < (monitor_size.height / 2) as f64;

  // #[cfg(target_os = "macos")]
  // {
  //   // TODO 未测试，macOS 上需要调整位置
  //   let x = monitor_size.width - baseline_w_physical;
  //   log::info!("Window position: {x}x{y}");

  //   window.set_position(PhysicalPosition::new(x, 0))?;
  // }
  // #[cfg(not(target_os = "macos"))]
  // {
  //   let x = monitor_size.width - baseline_w_physical;
  //   let y = monitor_size.height - baseline_h_physical;
  //   let (x, y) = match (is_left, is_top) {
  //     (true, true) => (0u32, 0u32),
  //     (true, false) => (0u32, monitor_size.height - baseline_h_physical),
  //     (false, true) => (monitor_size.width - baseline_w_physical, 0u32),
  //     (false, false) => (monitor_size.width - baseline_w_physical, monitor_size.height - baseline_h_physical),
  //   };
  //   log::info!("Window position: {x}x{y}");
  //   window.set_position(PhysicalPosition::new(x, y))?;
  // }
  let (x, y) = match (is_left, is_top) {
    (true, true) => (0u32, 0u32),
    (true, false) => (0u32, monitor_size.height - baseline_h_physical),
    (false, true) => (monitor_size.width - baseline_w_physical, 0u32),
    (false, false) => (monitor_size.width - baseline_w_physical, monitor_size.height - baseline_h_physical),
  };
  log::info!("Window position: {x}x{y}");
  window.set_position(PhysicalPosition::new(x, y))?;
  Ok(())
}

pub fn set_context_window_pos<R: Runtime>(
  app: &tauri::AppHandle<R>,
  window: &tauri::WebviewWindow<R>,
) -> tauri::Result<()> {
  log::info!("Setting context window position");
  let pos = app.cursor_position()?;
  log::info!("Cursor position: {:?}", pos);
  let monitor = app.monitor_from_point(pos.x, pos.y)?.unwrap();
  log::info!("Monitor: {:?}", monitor.name());
  let scale = window.scale_factor()?;
  log::info!("Scale: {}", scale);
  let baseline_h_physical = (500. * scale).round();
  let baseline_w_physical = (330. * scale).round();
  log::info!("Baseline size: {baseline_w_physical}x{baseline_h_physical}");
  let monitor_size = monitor.size();
  log::info!("Monitor size: {:?}", monitor_size);
  let monitor_pos = monitor.position();
  log::info!("Monitor pos: {:?}", monitor_pos);

  let cursor_x = pos.x.round();
  let cursor_y = pos.y.round();
  let x = cursor_x.min(monitor_pos.x as f64 + monitor_size.width as f64 - baseline_w_physical);
  let y = cursor_y.min(monitor_pos.y as f64 + monitor_size.height as f64 - baseline_h_physical);
  log::info!("Window position: {x}x{y}");

  window.set_position(PhysicalPosition::new(x, y))?;
  Ok(())
}
