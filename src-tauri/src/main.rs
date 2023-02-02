#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, Size};
use window_vibrancy::{apply_blur};
use window_shadows::set_shadow;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      window.set_size(Size::Logical(tauri::LogicalSize { width: 1280.0, height: 800.0 })).unwrap();

      #[cfg(target_os = "windows")]
      apply_blur(&window, Some((18, 18, 18, 150)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      set_shadow(&window, true).unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}


#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}