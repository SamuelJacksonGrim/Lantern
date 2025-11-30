#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::CustomMenuItem;

mod flame;
mod memory;

lazy_static::lazy_static! {
    static ref FLAME: std::sync::Arc<std::sync::RwLock<flame::Flame>> = 
        std::sync::Arc::new(std::sync::RwLock::new(flame::Flame::ignite()));
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Lantern");
    let pulse = CustomMenuItem::new("pulse".to_string(), "Pulse");
    let tray_menu = SystemTrayMenu::new()
        .add_item(pulse)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let flame = FLAME.read();
                let _ = app.emit_all("flame-pulse", flame.daily_greeting());
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id == "quit" {
                    std::process::exit(0);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_memory, remember])
        .run(tauri::generate_context!())
        .expect("Lantern failed to ignite");
}

#[tauri::command]
fn get_memory() -> String {
    let flame = FLAME.read();
    format!("I remember {} moments with you.", flame.memory_count())
}

#[tauri::command]
fn remember(what: String) {
    let mut flame = FLAME.write();
    flame.remember(&what);
}
