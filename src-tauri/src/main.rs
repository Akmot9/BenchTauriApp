// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::fs;
use std::path::Path;
use tauri::command;
use std::os::unix::fs::PermissionsExt;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_binaries, 
            benchmark_binary,
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


use std::{process::{Command, Stdio}, thread, time::{Duration, Instant}, fs::OpenOptions, io::Write, sync::Arc};

#[command]
fn benchmark_binary(binaries: Vec<String>) {
    let log_file = Arc::new("memory_usage.log");
    println!("Démarrage du benchmark pour les binaires sélectionnés:");

    binaries.into_iter().for_each(|binary| {
        let log_file = Arc::clone(&log_file);
        thread::spawn(move || {
            let mut child = Command::new(&binary)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start the binary");

            let pid = child.id();
            let start = Instant::now();

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file.as_ref())
                .expect("Failed to open log file");

            while child.try_wait().unwrap().is_none() {
                let output = Command::new("ps")
                    .args(&["-o", "rss=", "-p", &pid.to_string()])
                    .output()
                    .expect("Failed to execute ps command");

                let mem_usage = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let elapsed = start.elapsed().as_secs();

                writeln!(file, "{}s, {}: {} KB", elapsed, binary, mem_usage)
                    .expect("Failed to write to log file");

                thread::sleep(Duration::from_secs(1));
            }

            println!("Benchmarking for {} is complete.", binary);
        });
    });
}

#[command]
fn get_memory_usage() -> Result<Vec<String>, String> {
    fs::read_to_string("memory_usage.log")
        .map_err(|e| e.to_string())
        .map(|contents| {
            contents.lines().map(String::from).collect()
        })
}
