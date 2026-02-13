#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::{State, Manager, Window};

struct AppState {
    is_blocking: Arc<Mutex<bool>>,
    password: String,
}

#[tauri::command]
fn start_focus(state: State<'_, AppState>, apps: Vec<String>, window: Window) {
    let mut is_blocking = state.is_blocking.lock().unwrap();
    *is_blocking = true;

    // Window eka Fullscreen karanawa
    window.set_fullscreen(true).unwrap();
    window.set_always_on_top(true).unwrap();

    let is_blocking_clone = Arc::clone(&state.is_blocking);
    std::thread::spawn(move || {
        while *is_blocking_clone.lock().unwrap() {
            for app in &apps {
                // Windows wala app eka kill කරන command එක
                let _ = Command::new("taskkill")
                    .args(&["/F", "/IM", app])
                    .spawn();
            }
            std::thread::sleep(std::time::Duration::from_secs(2)); // Every 2 seconds check
        }
    });
}

#[tauri::command]
fn stop_focus(state: State<'_, AppState>, input_pass: String, window: Window) -> bool {
    if input_pass == state.password {
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
        .manage(AppState { 
            is_blocking: Arc::new(Mutex::new(false)),
            password: "123".to_string() 
        })
        .invoke_handler(tauri::generate_handler![start_focus, stop_focus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}