// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::apply_acrylic;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let system_tray = SystemTray::new();
    tauri::Builder::default().plugin(tauri_plugin_positioner::init()).system_tray(system_tray.clone()).on_system_tray_event(|app, event| {
        tauri_plugin_positioner::on_tray_event(app, &event);
        match event {
            SystemTrayEvent::LeftClick { 
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                let _ = window.move_window(Position::TrayCenter);

                #[cfg(target_os = "windows")]
                
                apply_acrylic(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            },
            _ => {},
        }
    })
        .invoke_handler(tauri::generate_handler![greet]).system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
