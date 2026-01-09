use tauri::{PhysicalPosition, Runtime};

pub fn set_tray_window_pos<R: Runtime>(
  app: &tauri::AppHandle<R>,
  window: &tauri::WebviewWindow<R>,
) -> tauri::Result<()> {
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

  #[cfg(target_os = "macos")]
  {
    // TODO 未测试，macOS 上需要调整位置
    let x = monitor_size.width - baseline_w_physical;
    log::info!("Window position: {x}x{y}");

    window.set_position(PhysicalPosition::new(x, 0))?;
  }
  #[cfg(not(target_os = "macos"))]
  {
    let x = monitor_size.width - baseline_w_physical;
    let y = monitor_size.height - baseline_h_physical;
    log::info!("Window position: {x}x{y}");

    window.set_position(PhysicalPosition::new(x, y))?;
  }
  Ok(())
}
