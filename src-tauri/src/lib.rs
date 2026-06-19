use tao::dpi::PhysicalPosition;

#[tauri::command]
fn center_cursor(window: tauri::Window) -> Result<(), String> {
  let size = window.inner_size().map_err(|e| e.to_string())?;
  let position = PhysicalPosition::new(size.width as f64 / 2.0, size.height as f64 / 2.0);
  window
    .set_cursor_position(position)
    .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![center_cursor])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
