// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::uninlined_format_args,
        clippy::cast_precision_loss,
        clippy::items_after_statements,
    )
)]
#![expect(clippy::doc_markdown, clippy::struct_excessive_bools)]

//! **Cross-platform installation and configuration system for NestGate**
//!
//! This crate provides comprehensive installation, configuration, and setup functionality
//! for NestGate storage management system. It includes platform-specific installers,
//! interactive setup wizards, and automated deployment capabilities.
//!
//! ## Overview
//!
//! NestGate Installer provides:
//! - **Cross-Platform Support**: Windows, macOS, Linux installation support
//! - **Interactive Setup**: Guided installation wizard with configuration
//! - **Automated Deployment**: Unattended installation for CI/CD and automation
//! - **Configuration Management**: System configuration and tuning
//! - **Dependency Management**: Automatic dependency resolution and installation
//! - **GUI & CLI Modes**: Both graphical and command-line installation options
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
//! │   Setup Wizard      │    │  Platform Detector  │    │  Download Manager   │
//! │   (Interactive)     │◄──►│  (OS & Hardware)    │◄──►│  (Binary Fetching)  │
//! └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
//!           │                           │                           │
//! ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
//! │   GUI Installer     │    │  Configuration      │    │  Installer Engine   │
//! │   (Graphical)       │    │  (System Settings)  │    │  (Core Logic)       │
//! └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
//!           │                           │                           │
//! ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
//! │   CLI Installer     │    │  Platform Support   │    │  Deployment Tools   │
//! │   (Command Line)    │    │  (OS Specific)      │    │  (Automated Setup)  │
//! └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
//! ```
//!
//! ## Key Features
//!
//! ### Cross-Platform Installation
//! - **Windows**: MSI packages, Registry integration, Service installation
//! - **macOS**: PKG installers, DMG mounting, LaunchDaemon integration
//! - **Linux**: DEB/RPM packages, Systemd services, Multi-distro support
//! - **Universal**: Tarball installations for unsupported platforms
//!
//! ### Interactive Setup Wizard
//! - **System Detection**: Automatic hardware and OS detection
//! - **Dependency Checking**: Verify and install required components
//! - **Configuration**: Interactive configuration with validation
//! - **Progress Tracking**: Real-time installation progress
//!
//! ### Configuration Management
//! - **System Tuning**: Automatic system optimization
//! - **Service Configuration**: Service installation and startup
//! - **Security Setup**: User permissions and security configuration
//! - **Network Configuration**: Port allocation and firewall setup
//!
//! ### Automated Deployment
//! - **Silent Installation**: Unattended installation with presets
//! - **CI/CD Integration**: Build and deployment pipeline support
//! - **Docker Support**: Containerized installation options
//! - **Configuration Templates**: Predefined setup configurations
//!
//! ## Quick Start
//!
//! Use the interactive or silent CLI flows documented under [Installation Modes](#installation-modes)
//! and [CLI Usage](#cli-usage). For a development build, run the installer binary via `cargo run`.
//!
//! ## Configuration Options
//!
//! Installation behavior is controlled through the [`config`] module and CLI flags. Paths, service
//! options, network ports, and security-related settings are applied according to platform support
//! and user choices during interactive or silent installation.
//!
//! Advanced scenarios may combine ZFS-related options, security hardening, and performance tuning
//! where the target platform supports them; see the [`config`] and [`installer`] modules for the
//! types and APIs exposed by this crate.
//!
//! ## Installation Components
//!
//! ### System Dependencies
//!
//! The installer automatically handles:
//! - **ZFS Installation**: OpenZFS kernel modules and utilities
//! - **Runtime Dependencies**: Required system libraries
//! - **Development Tools**: Build tools for native extensions
//! - **Database Setup**: Optional database installation
//! - **Monitoring Tools**: System monitoring and logging
//!
//! ### Service Configuration
//!
//! Service name, user, working directory, restart policy, and auto-start behavior are configured
//! during installation according to OS conventions (for example systemd on Linux).
//!
//! ### Network Setup
//!
//! API and related listen ports, bind addresses, firewall integration, and TLS options are
//! configured as part of installation when requested.
//!
//! ## Platform Support
//!
//! ### Linux Distributions
//! - **Ubuntu/Debian**: APT package management, systemd services
//! - **RHEL/CentOS/Fedora**: YUM/DNF package management, systemd services
//! - **Arch Linux**: Pacman package management, systemd services
//! - **SUSE/openSUSE**: Zypper package management, systemd services
//!
//! ### Windows Support
//! - **Service Installation**: Windows Service integration
//! - **Registry Configuration**: System registry setup
//! - **Firewall Configuration**: Windows Defender firewall rules
//! - **PowerShell Integration**: PowerShell cmdlets and modules
//!
//! ### macOS Support
//! - **Homebrew Integration**: Package management via Homebrew
//! - **LaunchDaemon**: System service integration
//! - **Security Framework**: macOS security and permissions
//! - **Code Signing**: Application signing and notarization
//!
//! ## Installation Modes
//!
//! ### Interactive Mode
//!
//! ```bash
//! # Run interactive installer
//! ./nestgate-installer
//!
//! # Or with Rust
//! cargo run --bin nestgate-installer
//! ```
//!
//! ### Silent Mode
//!
//! ```bash
//! # Automated installation with defaults
//! ./nestgate-installer --silent
//!
//! # With custom configuration
//! ./nestgate-installer --silent --config install-config.toml
//!
//! # With environment variables
//! NESTGATE_INSTALL_PATH=/opt/nestgate ./nestgate-installer --silent
//! ```
//!
//! ### Development Mode
//!
//! ```bash
//! # Development installation (no service setup)
//! ./nestgate-installer --dev-mode
//!
//! # Install from local build
//! ./nestgate-installer --local-build ./target/release/
//! ```
//!
//! ## Advanced Features
//!
//! ### Custom Download Sources
//!
//! Download mirrors, release channel, checksum verification, and proxy settings can be aligned
//! with your deployment policy; see [`download`] and [`installer`] for supported flows.
//!
//! ### Migration Support
//!
//! Upgrades from prior installations can preserve data and configuration where supported; use the
//! installer’s migration-related options and follow backup recommendations for production systems.
//!
//! ### Docker Integration
//!
//! Container-oriented installs may bundle images, registry references, and optional Compose
//! artifacts when your deployment uses Docker; see platform and packaging documentation for
//! details.
//!
//! ## Error Handling
//!
//! The installer surfaces errors for missing dependencies, insufficient permissions, unsupported
//! platforms, and other failure modes. Operators should retry after resolving dependency or
//! privilege issues and consult logs for diagnostic detail.
//!
//! ## Testing & Validation
//!
//! ### Installation Testing
//!
//! Run test suites and validation in isolated environments; use dry-run and mock modes before
//! applying changes to production hosts.
//!
//! ### Mock Installation
//!
//! ```bash
//! # Run installation in dry-run mode
//! ./nestgate-installer --dry-run
//!
//! # Test installation without system changes
//! NESTGATE_INSTALLER_MOCK=true ./nestgate-installer
//! ```
//!
//! ## Environment Variables
//!
//! ```bash
//! # Installation paths
//! NESTGATE_INSTALL_PATH=/opt/nestgate
//! NESTGATE_DATA_PATH=/var/lib/nestgate
//! NESTGATE_CONFIG_PATH=/etc/nestgate
//!
//! # Service configuration
//! NESTGATE_SERVICE_USER=nestgate
//! NESTGATE_API_PORT=8080
//!
//! # Installation options
//! NESTGATE_SILENT_INSTALL=true
//! NESTGATE_SKIP_DEPENDENCIES=false
//! NESTGATE_FORCE_REINSTALL=false
//!
//! # Development options
//! NESTGATE_DEV_MODE=false
//! NESTGATE_LOCAL_BUILD=/path/to/build
//! ```
//!
//! ## Module Organization
//!
//! ### Core Installation
//! - [`installer`] - Main installer implementation and logic
//! - [`config`] - Configuration management and validation
//! - [`platform`] - Platform detection and platform-specific operations
//! - [`wizard`] - Interactive installation wizard
//!
//! ### User Interface
//! - **GUI** — Graphical flows may be provided by the application binary; see [`wizard`] for the interactive CLI wizard.
//! - [`download`] - Binary and package download management
//!
//! ## CLI Usage
//!
//! ```bash
//! # Interactive installation
//! nestgate-installer
//!
//! # Silent installation
//! nestgate-installer --silent
//!
//! # Custom installation path
//! nestgate-installer --install-path /custom/path
//!
//! # Skip service setup
//! nestgate-installer --no-service
//!
//! # Development installation
//! nestgate-installer --dev-mode
//!
//! # Show help
//! nestgate-installer --help
//! ```
//!
//! ## Security Considerations
//!
//! - **Privilege Management**: Automatic privilege elevation when required
//! - **Secure Downloads**: Checksum verification and HTTPS-only downloads
//! - **User Isolation**: Service runs with minimal required permissions
//! - **File Permissions**: Appropriate file and directory permissions
//! - **Audit Trail**: Comprehensive installation logging
//!
//! ## Contributing
//!
//! See [`CONTRIBUTING.md`](../../../CONTRIBUTING.md) for development guidelines and how to contribute
//! to the NestGate installation system.

#[cfg(test)]
mod lib_tests;

pub mod config;
pub mod download;
pub mod error;
pub mod installer;
pub mod platform;
/// Interactive installation wizard and prompts.
pub mod wizard;

// Re-export commonly used types
pub use installer::NestGateInstaller as Installer;
pub use platform::PlatformInfo;
