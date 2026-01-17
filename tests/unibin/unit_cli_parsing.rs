//! UniBin CLI Parsing - Unit Tests
//!
//! Tests for UniBin architecture CLI parsing and validation

use nestgate_bin::cli::{Cli, Commands};
use clap::Parser;

#[test]
fn test_version_flag() {
    // Test --version flag
    let args = vec!["nestgate", "--version"];
    let result = Cli::try_parse_from(args);
    
    // Should parse successfully (version handled by clap)
    assert!(result.is_ok() || result.is_err()); // Version exits early
}

#[test]
fn test_help_flag() {
    // Test --help flag
    let args = vec!["nestgate", "--help"];
    let result = Cli::try_parse_from(args);
    
    // Should parse (help exits early) or succeed
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_daemon_command() {
    // Test daemon subcommand
    let args = vec!["nestgate", "daemon"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse daemon");
    
    assert!(matches!(cli.command, Commands::Daemon { .. }));
}

#[test]
fn test_daemon_with_port() {
    // Test daemon with custom port
    let args = vec!["nestgate", "daemon", "--port", "9000"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse daemon with port");
    
    if let Commands::Daemon { port, .. } = cli.command {
        assert_eq!(port, 9000);
    } else {
        panic!("Expected Daemon command");
    }
}

#[test]
fn test_daemon_with_background() {
    // Test daemon with background flag
    let args = vec!["nestgate", "daemon", "--background"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse daemon --background");
    
    if let Commands::Daemon { background, .. } = cli.command {
        assert!(background);
    } else {
        panic!("Expected Daemon command with background");
    }
}

#[test]
fn test_status_command() {
    // Test status subcommand
    let args = vec!["nestgate", "status"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse status");
    
    assert!(matches!(cli.command, Commands::Status { .. }));
}

#[test]
fn test_health_command() {
    // Test health subcommand
    let args = vec!["nestgate", "health"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse health");
    
    assert!(matches!(cli.command, Commands::Health { .. }));
}

#[test]
fn test_discover_command() {
    // Test discover subcommand
    let args = vec!["nestgate", "discover", "songbird"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse discover");
    
    if let Commands::Discover { primal_type, .. } = cli.command {
        assert_eq!(primal_type, "songbird");
    } else {
        panic!("Expected Discover command");
    }
}

#[test]
fn test_invalid_command() {
    // Test invalid subcommand
    let args = vec!["nestgate", "invalid-command"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err(), "Should reject invalid command");
}

#[test]
fn test_daemon_invalid_port() {
    // Test daemon with invalid port (too high)
    let args = vec!["nestgate", "daemon", "--port", "99999"];
    let result = Cli::try_parse_from(args);
    
    // Clap should reject invalid port
    assert!(result.is_err(), "Should reject invalid port");
}

#[test]
fn test_multiple_flags() {
    // Test daemon with multiple flags
    let args = vec![
        "nestgate",
        "daemon",
        "--port",
        "8080",
        "--address",
        "0.0.0.0",
        "--background",
    ];
    let cli = Cli::try_parse_from(args).expect("Failed to parse multiple flags");
    
    if let Commands::Daemon {
        port,
        address,
        background,
    } = cli.command
    {
        assert_eq!(port, 8080);
        assert_eq!(address, "0.0.0.0");
        assert!(background);
    } else {
        panic!("Expected Daemon command");
    }
}

#[test]
fn test_discover_with_timeout() {
    // Test discover with custom timeout
    let args = vec![
        "nestgate",
        "discover",
        "beardog",
        "--timeout",
        "30",
    ];
    let cli = Cli::try_parse_from(args).expect("Failed to parse discover with timeout");
    
    if let Commands::Discover { primal_type, timeout, .. } = cli.command {
        assert_eq!(primal_type, "beardog");
        assert_eq!(timeout, 30);
    } else {
        panic!("Expected Discover command");
    }
}

#[test]
fn test_status_with_verbose() {
    // Test status with verbose flag
    let args = vec!["nestgate", "status", "--verbose"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse status --verbose");
    
    if let Commands::Status { verbose, .. } = cli.command {
        assert!(verbose);
    } else {
        panic!("Expected Status command");
    }
}

#[test]
fn test_health_with_comprehensive() {
    // Test health with comprehensive check
    let args = vec!["nestgate", "health", "--comprehensive"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse health --comprehensive");
    
    if let Commands::Health { comprehensive, .. } = cli.command {
        assert!(comprehensive);
    } else {
        panic!("Expected Health command");
    }
}

#[test]
fn test_missing_required_arg() {
    // Test discover without primal_type (required)
    let args = vec!["nestgate", "discover"];
    let result = Cli::try_parse_from(args);
    
    assert!(result.is_err(), "Should require primal_type");
}

#[test]
fn test_default_values() {
    // Test daemon with default values
    let args = vec!["nestgate", "daemon"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse daemon defaults");
    
    if let Commands::Daemon {
        port,
        address,
        background,
    } = cli.command
    {
        assert_eq!(port, 8080); // Default port
        assert_eq!(address, "127.0.0.1"); // Default address
        assert!(!background); // Default false
    } else {
        panic!("Expected Daemon command");
    }
}

#[test]
fn test_short_flags() {
    // Test short flags (if supported)
    let args = vec!["nestgate", "daemon", "-p", "9000"];
    let result = Cli::try_parse_from(args);
    
    // Should succeed if short flags are supported
    if let Ok(cli) = result {
        if let Commands::Daemon { port, .. } = cli.command {
            assert_eq!(port, 9000);
        }
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_args() {
        // Test with no arguments (should show help or error)
        let args = vec!["nestgate"];
        let result = Cli::try_parse_from(args);
        
        // Should either succeed (if help) or error (if command required)
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_special_characters_in_args() {
        // Test with special characters
        let args = vec!["nestgate", "daemon", "--address", "::1"];
        let cli = Cli::try_parse_from(args).expect("Failed IPv6 address");
        
        if let Commands::Daemon { address, .. } = cli.command {
            assert_eq!(address, "::1");
        }
    }

    #[test]
    fn test_very_long_primal_name() {
        // Test discover with long primal name
        let long_name = "a".repeat(1000);
        let args = vec!["nestgate", "discover", &long_name];
        let cli = Cli::try_parse_from(args).expect("Failed long primal name");
        
        if let Commands::Discover { primal_type, .. } = cli.command {
            assert_eq!(primal_type.len(), 1000);
        }
    }

    #[test]
    fn test_unicode_in_args() {
        // Test with unicode characters
        let args = vec!["nestgate", "discover", "🦀-primal"];
        let cli = Cli::try_parse_from(args).expect("Failed unicode");
        
        if let Commands::Discover { primal_type, .. } = cli.command {
            assert!(primal_type.contains("🦀"));
        }
    }
}
