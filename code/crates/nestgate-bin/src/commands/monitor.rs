// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Performance monitoring commands
//!
//! ✅ EVOLVED: Real implementation replacing println stub
//! Provides real-time system and storage monitoring with periodic sampling.

use anyhow::Result;
use std::path::PathBuf;

/// Execute the performance monitoring command
pub async fn execute(interval: u64, output: Option<PathBuf>, duration: Option<u64>) -> Result<()> {
    println!("📊 NestGate Performance Monitor");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Interval:  {}s", interval);
    if let Some(ref path) = output {
        println!("  Output:    {}", path.display());
    }
    if let Some(dur) = duration {
        println!("  Duration:  {}s", dur);
    } else {
        println!("  Duration:  continuous (Ctrl+C to stop)");
    }
    println!();

    let start = std::time::Instant::now();
    let mut sample_count = 0u64;
    let mut output_file = if let Some(ref path) = output {
        Some(tokio::fs::File::create(path).await?)
    } else {
        None
    };

    // Write CSV header
    if let Some(ref mut file) = output_file {
        use tokio::io::AsyncWriteExt;
        file.write_all(b"timestamp,cpu_cores,socket_alive,backend,storage_exists,sample_num\n")
            .await?;
    }

    loop {
        // Check duration limit
        if let Some(dur) = duration {
            if start.elapsed().as_secs() >= dur {
                println!(
                    "\n⏱️  Duration limit reached ({:.0}s)",
                    start.elapsed().as_secs_f64()
                );
                break;
            }
        }

        sample_count += 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // ✅ REAL: Gather system metrics
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        // ✅ REAL: Check socket liveness
        let socket_alive = match nestgate_core::rpc::SocketConfig::from_environment() {
            Ok(config) => {
                if config.socket_path.exists() {
                    tokio::net::UnixStream::connect(&config.socket_path)
                        .await
                        .is_ok()
                } else {
                    false
                }
            }
            Err(_) => false,
        };

        // ✅ REAL: Backend detection
        let caps = nestgate_core::services::storage::capabilities::detect_backend();

        // ✅ REAL: Storage path check
        let storage_path = std::env::var("NESTGATE_STORAGE_PATH")
            .unwrap_or_else(|_| "/var/lib/nestgate/storage".to_string());
        let storage_exists = std::path::Path::new(&storage_path).exists();

        // Display
        println!("[{}] Sample #{}", timestamp, sample_count);
        println!("  CPU Cores:  {}", cpu_count);
        println!(
            "  Socket:     {}",
            if socket_alive {
                "✅ ALIVE"
            } else {
                "⏸️  DOWN"
            }
        );
        println!("  Backend:    {:?}", caps.backend_type);
        println!(
            "  Storage:    {} ({})",
            storage_path,
            if storage_exists { "exists" } else { "missing" }
        );

        // Write to file if specified
        if let Some(ref mut file) = output_file {
            use tokio::io::AsyncWriteExt;
            let line = format!(
                "{},{},{},{:?},{},{}\n",
                timestamp, cpu_count, socket_alive, caps.backend_type, storage_exists, sample_count
            );
            file.write_all(line.as_bytes()).await?;
        }

        // Sleep for interval
        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }

    if let Some(ref path) = output {
        println!("📝 Metrics written to: {}", path.display());
    }
    println!("📊 Total samples: {}", sample_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor_with_short_duration() {
        // Run monitor for 1 second with 1s interval
        let result = execute(1, None, Some(1)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_with_csv_output() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test_metrics.csv");

        let result = execute(1, Some(output_path.clone()), Some(1)).await;
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Verify CSV has header and at least one data row
        let content = tokio::fs::read_to_string(&output_path).await.unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert!(lines.len() >= 2, "Should have header + at least 1 data row");
        assert!(lines[0].contains("timestamp"));
        assert!(lines[0].contains("cpu_cores"));
    }
}
