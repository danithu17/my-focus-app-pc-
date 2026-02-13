#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::{State, Manager, Window};

struct AppState {
    is_blocking: Arc<Mutex<bool>>,
}

#[tauri::command]
fn start_focus(state: State<'_, AppState>, apps: Vec<String>, window: Window) {
    let mut is_blocking = state.is_blocking.lock().unwrap();
    *is_blocking = true;

    // Window settings
    window.set_fullscreen(true).unwrap();
    window.set_always_on_top(true).unwrap();

    let is_blocking_clone = Arc::clone(&state.is_blocking);
    std::thread::spawn(move || {
        while *is_blocking_clone.lock().unwrap() {
            for app in &apps {
                #[cfg(target_os = "windows")]
                let _ = Command::new("taskkill")
                    .args(&["/F", "/IM", app])
                    .spawn();
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}

#[tauri::command]
fn stop_focus(state: State<'_, AppState>, input_pass: String, window: Window) -> bool {
    // Password eka "123" kiyala mama damma
    if input_pass == "123" {
        let mut is_blocking = state.is_blocking.lock().unwrap();
        *is_blocking = false;
        
        window.set_fullscreen(false).unwrap();
        window.set_always_on_top(false).unwrap();
        return true;
    }
    false
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { is_blocking: Arc::new(Mutex::new(false)) })
        .invoke_handler(tauri::generate_handler![start_focus, stop_focus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}