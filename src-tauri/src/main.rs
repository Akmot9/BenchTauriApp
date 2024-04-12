// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod benchstate;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use std::sync::{mpsc, Mutex};
use benchstate::{BenchState, Statistic};
use tauri::{command, AppHandle, Manager};
use std::os::unix::fs::PermissionsExt;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(BenchState::default()))
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


use std::{process::{Command, Stdio}, thread, time::Duration};



#[command]
async fn benchmark_binary(binaries: Vec<String>, app: AppHandle) {
    println!("Démarrage du benchmark pour les binaires sélectionnés...");

    let (tx, rx) = mpsc::channel();
    let mut emitter_handles = vec![];

    // Thread récepteur
    let app_for_thread = app.clone();
    let receiver_handle = thread::spawn(move || {
        println!("Thread récepteur démarré.");
        let state = app_for_thread.state::<Mutex<BenchState>>();
        for stat in rx {
            println!("Réception d'une stat: {:?}", stat);
            let mut state_guard = state.lock().unwrap();
            state_guard.add_statistic(stat);
        }
        println!("Thread récepteur terminé.");
        println!("Statistiques: {}", state.lock().unwrap().statistics);
    });

    // Threads émetteurs pour chaque binaire
    for binary in binaries.into_iter() {
        let tx_clone = tx.clone();
        println!("Démarrage du benchmark pour '{}'", binary);
        let handle = thread::spawn(move || {
            let mut output = Command::new(&binary)
                .stdout(Stdio::null())
                .spawn()
                .expect("Échec du démarrage du binaire");

            let pid = output.id();
            println!("Binaire '{}' démarré avec PID: {}", binary, pid);

            loop {
                println!("boucle {:?}", output);
                if let Ok(None) = output.try_wait() {

                    let output = Command::new("ps")
                        .args(&["-o", "rss,%mem,vsz,%cpu,ni=", "-p", &pid.to_string()])
                        .output()
                        .expect("Échec de l'exécution de la commande ps");

                    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

                    if !output_str.is_empty() {
                        let parts: Vec<&str> = output_str.split_whitespace().collect();
                        match parse_statistic(&parts, &binary) {
                            Ok(stat) => {
                                println!("Sending stat for '{}': {}", binary, stat);
                                tx_clone.send(stat).expect("Failed to send stat");
                            },
                            Err(e) => println!("Error parsing statistic: {}", e),
                        }
                    }
                    thread::sleep(Duration::from_secs(1));
                } else {
                    println!("Binaire '{}' terminé.", binary);
                    break;
                }
            }
        });
        emitter_handles.push(handle);
    }

    // Attendre que tous les threads émetteurs terminent
    for handle in emitter_handles {
        handle.join().expect("Un thread émetteur a paniqué.");
    }
    println!("Tous les benchmarks binaires sont terminés.");

    drop(tx); // Fermer le canal en laissant tomber l'expéditeur
    receiver_handle.join().expect("Le thread récepteur a paniqué.");
    println!("Benchmarking et réception des statistiques terminés.");
}

fn parse_statistic(parts: &[&str], binary: &str) -> Result<Statistic, String> {
    if parts.len() >= 5 {
        let len = parts.len();
        let rss = parts[len - 5].parse::<u64>()
            .map_err(|e| format!("Error parsing RSS: {}", e))?;
        let mem = parts[len - 4].parse::<f32>()
            .map_err(|e| format!("Error parsing %MEM: {}", e))?;
        let vsz = parts[len - 3].parse::<u64>()
            .map_err(|e| format!("Error parsing VSZ: {}", e))?;
        let cpu = parts[len - 2].parse::<f32>()
            .map_err(|e| format!("Error parsing %CPU: {}", e))?;
        let ni = parts[len - 1].parse::<i32>()
            .map_err(|e| format!("Error parsing NI: {}", e))?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Error calculating time: {}", e))?
            .as_secs(); // Gets the current time in seconds since the Unix Epoch


        Ok(Statistic {
            name: binary.to_string(),
            rss, mem, vsz, cpu, ni
        })
    } else {
        Err("Not enough data to parse statistic".to_string())
    }
}


#[command]
fn get_memory_usage() -> Result<Vec<String>, String> {
    fs::read_to_string("memory_usage.log")
        .map_err(|e| e.to_string())
        .map(|contents| {
            contents.lines().map(String::from).collect()
        })
}
