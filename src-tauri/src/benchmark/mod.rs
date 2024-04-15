// benchmark.rs

use std::sync::{mpsc, Mutex};
use std::thread;
use std::process::{Command, Stdio};
use std::time::Duration;
use serde_json::json;
use tauri::{AppHandle, Manager, Window};

use crate::benchstate::{BenchState, Statistic};


// Starts the benchmark process for each binary.
pub fn benchmark_binary(binaries: Vec<String>, app: AppHandle, window: Window) {
    println!("Starting benchmark for selected binaries: {:?}", &binaries);
    let (tx, rx) = mpsc::channel();
    let mut emitter_handles = vec![];

    let receiver_handle = start_receiver_thread(app.clone(), window.clone(), rx);

    for binary in binaries {
        emitter_handles.push(start_emitter_thread(binary, tx.clone()));
    }

    wait_for_emitter_threads(emitter_handles);
    drop(tx);  // Close the channel by dropping the transmitter
    wait_for_receiver_thread(receiver_handle);
}

// Waits for all emitter threads to complete.
fn wait_for_emitter_threads(emitter_handles: Vec<thread::JoinHandle<()>>) {
    for handle in emitter_handles {
        handle.join().expect("An emitter thread has panicked.");
    }
    println!("All emitter threads have completed.");
}


// Waits for the receiver thread to complete.
fn wait_for_receiver_thread(receiver_handle: thread::JoinHandle<()>) {
    receiver_handle.join().expect("The receiver thread has panicked.");
    println!("Receiver thread has terminated.");
}




// Sets up the receiver thread that listens for statistics and updates the UI.
fn start_receiver_thread(app: AppHandle, window: Window, rx: mpsc::Receiver<Statistic>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let state = app.state::<Mutex<BenchState>>();
        while let Ok(stat) = rx.recv() {
            println!("Received stat: {:?}", stat);
            let mut state_guard = state.lock().unwrap();
            state_guard.add_statistic(stat);

            let stats_json = json!(state_guard.statistics);
            window.emit("update_statistics", stats_json)
                .expect("Failed to send statistics");
        }
        println!("Receiver thread terminated.");
    })
}


// Starts the emitter thread for a given binary that monitors and sends its statistics.
fn start_emitter_thread(binary: String, tx: mpsc::Sender<Statistic>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        benchmark_single_binary(binary, tx);
    })
}

fn benchmark_single_binary(binary: String, tx: mpsc::Sender<Statistic>) {
    println!("Starting benchmark for '{}'", binary);
    let child = Command::new(&binary)
        .stdout(Stdio::null())
        .spawn();

    if let Ok(child) = child {
        monitor_process(child, binary, tx);
    } else {
        println!("Failed to start '{}'.", binary);
    }
}

// Monitors the process and sends statistics via the provided sender.
fn monitor_process(mut child: std::process::Child, binary: String, tx: mpsc::Sender<Statistic>) {
    println!("Binary '{}' started with PID: {}", binary, child.id());
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break, // Process has exited
            Ok(None) => {
                let stats = fetch_process_stats(child.id(), &binary);
                if let Ok(stat) = stats {
                    tx.send(stat).expect("Failed to send stat");
                }
                thread::sleep(Duration::from_secs(1));
            }
            Err(_) => break, // Process handling error
        }
    }
    send_termination_stat(&binary, tx);
}


// Fetches process statistics using the 'ps' command.
fn fetch_process_stats(pid: u32, binary: &str) -> Result<Statistic, String> {
    let output = Command::new("ps")
        .args(&["-o", "rss,%mem,vsz,%cpu,ni=", "-p", &pid.to_string()])
        .output()
        .expect("Failed to execute 'ps' command");
    parse_statistic(&String::from_utf8_lossy(&output.stdout).trim().split_whitespace().collect::<Vec<&str>>(), binary)
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

        Ok(Statistic {
            name: binary.to_string(),
            rss, mem, vsz, cpu, ni
        })
    } else {
        Err("Not enough data to parse statistic".to_string())
    }
}

// Sends a termination statistic indicating the process has ended.
fn send_termination_stat(binary: &str, tx: mpsc::Sender<Statistic>) {
    tx.send(Statistic {
        name: binary.to_string(),
        rss: 0,
        mem: 0.0,
        vsz: 0,
        cpu: 0.0,
        ni: 0,
    }).expect("Failed to send termination stat");
}