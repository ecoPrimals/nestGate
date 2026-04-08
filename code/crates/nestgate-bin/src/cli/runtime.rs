// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Logging setup and CLI banner output.

/// Setup logging based on CLI arguments
pub fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(format!("nestgate={level}"))
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .try_init();
}

/// Print welcome banner
pub fn print_banner() {
    println!(
        "🏠 NestGate v{} - Universal ZFS & Storage Management",
        env!("CARGO_PKG_VERSION")
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🌟 ZFS features on ANY storage backend");
    println!("📦 Local, Cloud, Network, Memory support");
    println!("⚡ Production-ready performance");
    println!("🔒 Enterprise-grade data integrity");
    println!();
}
