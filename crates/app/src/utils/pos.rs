pub fn calc_pane_pos(
  pane: tauri::PhysicalSize<u32>,
  win: tauri::Monitor,
  cursor: tauri::PhysicalPosition<f64>,
) -> tauri::PhysicalPosition<f64> {
  log::info!("获取屏幕尺寸");
  let size = win.size();
  log::info!("计算窗口位置");
  let x = cursor.x - (pane.width as f64) / 2.;
  let y = cursor.y - (pane.height as f64);
  let max_x = (size.width as f64) - pane.width as f64;
  let max_y = (size.height as f64) - pane.height as f64;
  tauri::PhysicalPosition::<f64>::new(x.min(max_x), y.min(max_y))
}
