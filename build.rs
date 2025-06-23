use std::env;
use std::process::Command;

fn main() {
    // Only run checks in debug mode
    if env::var("PROFILE").unwrap() == "debug" {
        // Run clippy checks
        if !Command::new("cargo")
            .args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
            .status()
            .expect("Failed to run clippy")
            .success()
        {
            panic!("Clippy checks failed");
        }

        // Run rustfmt checks
        if !Command::new("cargo")
            .args(["fmt", "--all", "--check"])
            .status()
            .expect("Failed to run rustfmt")
            .success()
        {
            panic!("Rustfmt checks failed");
        }
    }

    // Set up feature flags
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_USE_SYSTEM_LIBS");
    
    // Link system libraries if needed
    if env::var_os("CARGO_FEATURE_USE_SYSTEM_LIBS").is_some() {
        println!("cargo:rustc-link-lib=zfs");
        println!("cargo:rustc-link-lib=nvpair");
    }

    // Set up conditional compilation
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-cfg=target_linux");
    }
} 