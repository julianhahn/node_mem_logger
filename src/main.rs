use chrono::Local;
use clap::{command, Parser};
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command()]
struct Args {
    /// Number of processes to show per log (excluding frontend)
    #[arg(short = 'p', long, default_value_t = 5)]
    process_count: usize,

    /// Number of log entries to keep
    #[arg(short = 'l', long, default_value_t = 10)]
    log_entries: usize,

    /// Seconds between log entries
    #[arg(short = 's', long, default_value_t = 10)]
    interval_seconds: u64,
}

struct ProcessInfo {
    pid: String,
    user: String,
    memory_kb: u64,
    cpu_percent: f32,
    command: String,
}

fn format_memory(kb: u64) -> String {
    if kb > 1_048_576 {
        format!("{:.2} GB", kb as f64 / 1_048_576.0)
    } else if kb > 1024 {
        format!("{:.2} MB", kb as f64 / 1024.0)
    } else {
        format!("{} KB", kb)
    }
}

fn get_node_processes() -> Vec<ProcessInfo> {
    let ps_output = Command::new("ps")
        .arg("aux")
        .output()
        .expect("Failed to execute ps command");

    let output_str = String::from_utf8_lossy(&ps_output.stdout);

    let mut processes: Vec<ProcessInfo> = output_str
        .lines()
        .filter(|line| {
            line.contains("node") && !line.contains("grep") && !line.contains("node_mem_log")
        })
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let memory_str = parts.get(5).unwrap_or(&"0").to_string();
            let memory_kb = memory_str.parse::<u64>().unwrap_or(0);
            let cpu_str = parts.get(2).unwrap_or(&"0").to_string();
            let cpu_percent = cpu_str.parse::<f32>().unwrap_or(0.0);

            let command = if parts.len() > 10 {
                parts[10..].join(" ")
            } else {
                "unknown".to_string()
            };

            ProcessInfo {
                user: parts.get(0).unwrap_or(&"unknown").to_string(),
                pid: parts.get(1).unwrap_or(&"unknown").to_string(),
                cpu_percent,
                memory_kb,
                command,
            }
        })
        .collect();

    processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
    processes
}

fn create_log_filename() -> String {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    format!("node_monitor_{}.log", timestamp)
}

fn write_log_entry(
    entries: &mut VecDeque<String>,
    file: &mut File,
    new_entry: String,
    max_entries: usize,
) {
    entries.push_back(new_entry);
    if entries.len() > max_entries {
        entries.pop_front();
    }

    // Clear file and write all entries
    file.set_len(0).expect("Failed to truncate file");
    file.seek(SeekFrom::Start(0))
        .expect("Failed to seek to start");
    for entry in entries {
        writeln!(file, "{}", entry).expect("Failed to write entry");
    }
    file.flush().expect("Failed to flush file");
}

fn main() {
    let args = Args::parse();
    let filename = create_log_filename();
    println!("Starting monitoring with:");
    println!("  - Processes per log: {}", args.process_count);
    println!("  - Log entries kept: {}", args.log_entries);
    println!("  - Interval: {} seconds", args.interval_seconds);
    println!("Logging to: {}", filename);

    let mut entries = VecDeque::new();

    loop {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&filename)
            .expect("Failed to open log file");

        let timestamp = Local::now();
        let mut log_entry = format!(
            "\n=== Node.js Processes Check: {} ===",
            timestamp.format("%Y-%m-%d %H:%M:%S")
        );

        let processes = get_node_processes();

        // Frontend process check (always shown)
        let frontend = processes.iter().find(|p| {
            p.command.contains("yarn start")
                || p.command.contains("yarn dev")
                || p.command.contains("/bin/yarn start")
                || p.command.contains("/bin/yarn dev")
        });

        log_entry.push_str("\n\nFrontend Process:\n");
        match frontend {
            Some(p) => log_entry.push_str(&format!(
                "PID: {} | Memory: {} | CPU: {:.1}% | {}\n",
                p.pid,
                format_memory(p.memory_kb),
                p.cpu_percent,
                p.command
            )),
            None => log_entry.push_str("No frontend process (yarn start/dev) found!\n"),
        }

        log_entry.push_str("\nTop Memory Node Processes:\n");
        for process in processes.iter().take(args.process_count) {
            log_entry.push_str(&format!(
                "PID: {} | Memory: {} | CPU: {:.1}% | {}\n",
                process.pid,
                format_memory(process.memory_kb),
                process.cpu_percent,
                process.command
            ));
        }

        write_log_entry(&mut entries, &mut file, log_entry, args.log_entries);
        thread::sleep(Duration::from_secs(args.interval_seconds));
    }
}
