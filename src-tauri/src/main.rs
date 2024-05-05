// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate lazy_static;

mod swtor_hook;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      swtor_hook::start_swtor_hook,
      swtor_hook::is_hooked_in,
      swtor_hook::submit_actual_post
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
