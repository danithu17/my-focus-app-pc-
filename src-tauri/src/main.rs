#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::State;

struct AppState {
    is_blocking: Arc<Mutex<bool>>,
}

#[tauri::command]
fn toggle_focus(state: State<'_, AppState>, apps: Vec<String>, status: bool) {
    let mut is_blocking = state.is_blocking.lock().unwrap();
    *is_blocking = status;

    if *is_blocking {
        let is_blocking_clone = Arc::clone(&state.is_blocking);
        std::thread::spawn(move || {
            while *is_blocking_clone.lock().unwrap() {
                for app in &apps {
                    // Windows command to kill the app process
                    let _ = Command::new("taskkill")
                        .args(&["/F", "/IM", app])
                        .spawn();
                }
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        });
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { is_blocking: Arc::new(Mutex::new(false)) })
        .invoke_handler(tauri::generate_handler![toggle_focus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}