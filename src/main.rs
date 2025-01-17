use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

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

fn main() {
    let filename = create_log_filename();
    println!("Starting monitoring, logging to: {}", filename);

    loop {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)
            .expect("Failed to open log file");

        let timestamp = Local::now();
        writeln!(
            file,
            "\n=== Node.js Processes Check: {} ===",
            timestamp.format("%Y-%m-%d %H:%M:%S")
        )
        .expect("Failed to write timestamp");

        let processes = get_node_processes();

        // First check for frontend process
        let frontend = processes.iter().find(|p| {
            p.command.contains("yarn start")
                || p.command.contains("yarn dev")
                || p.command.contains("/bin/yarn start")
                || p.command.contains("/bin/yarn dev")
        });

        writeln!(file, "\nFrontend Process:").expect("Failed to write");
        match frontend {
            Some(p) => writeln!(
                file,
                "PID: {} | Memory: {} | CPU: {:.1}% | {}",
                p.pid,
                format_memory(p.memory_kb),
                p.cpu_percent,
                p.command
            )
            .expect("Failed to write frontend info"),
            None => writeln!(file, "No frontend process (yarn start/dev) found!")
                .expect("Failed to write frontend info"),
        }

        writeln!(file, "\nTop Memory Node Processes:").expect("Failed to write");
        for process in processes.iter().take(5) {
            writeln!(
                file,
                "PID: {} | Memory: {} | CPU: {:.1}% | {}",
                process.pid,
                format_memory(process.memory_kb),
                process.cpu_percent,
                process.command
            )
            .expect("Failed to write process info");
        }

        thread::sleep(Duration::from_secs(10));
    }
}
