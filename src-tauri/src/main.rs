// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod benchstate;

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use benchstate::BenchState;
use tauri::{command, AppHandle, Window};
use std::os::unix::fs::PermissionsExt;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(BenchState::default()))
        .invoke_handler(tauri::generate_handler![
            list_binaries, 
            benchmark,
            get_memory_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn list_binaries() -> Vec<String> {
    let mut binaries = Vec::new();
    if let Ok(entries) = fs::read_dir("/usr/bin") {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if is_binary(&path) {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    binaries.push(name.to_string());
                }
            }
        }
    }
    binaries
}

fn is_binary(path: &Path) -> bool {
    // Simplistic check to see if a file is binary. You might want to extend this!
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
    } else {
        false
    }
}



mod benchmark;
use benchmark::benchmark_binary;

#[command]
async fn benchmark(binaries: Vec<String>, app: AppHandle, window: Window) {
    benchmark_binary(binaries, app, window);
}


#[command]
fn get_memory_usage() -> Result<Vec<String>, String> {
    fs::read_to_string("memory_usage.log")
        .map_err(|e| e.to_string())
        .map(|contents| {
            contents.lines().map(String::from).collect()
        })
}
