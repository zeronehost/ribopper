use tauri::PhysicalRect;

pub fn calc_pane_pos(
  pane: tauri::PhysicalSize<u32>,
  win: tauri::Monitor,
) -> tauri::PhysicalPosition<f64> {
  log::info!("获取屏幕尺寸");
  let PhysicalRect { size, .. } = win.work_area();
  log::info!("计算窗口位置");
  let max_x = (size.width as f64) - pane.width as f64;
  let max_y = (size.height as f64) - pane.height as f64;
  tauri::PhysicalPosition::<f64>::new(max_x, max_y)
}
