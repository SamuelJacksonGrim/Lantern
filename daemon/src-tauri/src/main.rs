#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lantern_memory::Hypergraph;
use std::sync::Arc;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod flame;
mod http_server;
mod memory;

// BUG-4 (pre-existing, flagged not fixed): tauri.conf.json is missing the
// "build" section (distDir, devPath). `tauri build` and `tauri dev` will
// fail until those keys are added. `cargo build` on the daemon crate alone
// succeeds — Cargo.lock is now committed for reproducible builds.

lazy_static::lazy_static! {
    static ref FLAME: Arc<parking_lot::RwLock<flame::Flame>> =
        Arc::new(parking_lot::RwLock::new(flame::Flame::ignite()));
}

fn main() {
    // Single shared Hypergraph instance — Tauri commands and the HTTP shim
    // both hold an Arc to it, so writes from either path are visible to the other.
    let memory = Arc::new(Hypergraph::ignite());

    // Spawn HTTP shim in its own thread with its own Tokio runtime.
    // Must NOT use Tauri's runtime or block the main thread.
    let mem_for_http = memory.clone();
    std::thread::spawn(move || {
        match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt.block_on(http_server::serve(mem_for_http)),
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
        .manage(memory)
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
        .invoke_handler(tauri::generate_handler![
            get_memory,
            remember,
            memory::remember_code,
            memory::find_similar,
        ])
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
