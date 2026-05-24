#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::CustomMenuItem;

mod flame;
mod memory;
mod http_server;

// BUG-4 (pre-existing, flagged not fixed): tauri.conf.json is missing the
// "build" section (distDir, devPath). `tauri build` and `tauri dev` will
// fail until those keys are added. `cargo build` on the Rust crate alone
// succeeds, but the full Tauri toolchain does not. Also: no Cargo.lock
// exists in the repo (never been compiled end-to-end). Fix when setting up
// the build pipeline.

lazy_static::lazy_static! {
    static ref FLAME: std::sync::Arc<std::sync::RwLock<flame::Flame>> =
        std::sync::Arc::new(std::sync::RwLock::new(flame::Flame::ignite()));
}

fn main() {
    // Spawn HTTP shim in its own thread with its own Tokio runtime.
    // Must NOT use Tauri's runtime or block the main thread.
    std::thread::spawn(|| {
        match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt.block_on(http_server::start()),
            Err(e) => eprintln!("[LANTERN] HTTP shim Tokio runtime failed: {e}. Memory backbone disabled."),
        }
    });

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
        .invoke_handler(tauri::generate_handler![get_memory, remember, memory::remember_code, memory::find_similar])
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
