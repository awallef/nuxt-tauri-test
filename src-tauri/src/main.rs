#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::echo;
use commands::unzip_file;
use commands::zip_directory;
use commands::zip_file;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            unzip_file,
            zip_file,
            zip_directory,
            echo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
