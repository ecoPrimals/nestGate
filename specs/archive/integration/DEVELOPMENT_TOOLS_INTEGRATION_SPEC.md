---
title: Development Tools Integration Specification
description: Detailed specification for integrating GitClone development tools into v2 orchestrator-centric architecture
version: 2.0.0
date: 2025-01-26
status: Integration Specification
---

# Development Tools Integration Specification

## Overview

This specification details the integration of development tools and infrastructure from the `nestgate-gitclone` repository into our v2 orchestrator-centric architecture. The integration will streamline development workflow, improve build processes, and enhance developer experience.

## Integration Objectives

### Primary Goals
1. **Streamlined Development**: Provide efficient development environment setup and management
2. **Enhanced Build System**: Integrate sophisticated workspace configuration and dependency management
3. **Improved CLI**: Adapt CLI tools for v2 orchestrator-centric operations
4. **Developer Experience**: Minimize setup time and maximize productivity

### Success Criteria
- ✅ New developers can setup environment in <10 minutes
- ✅ Full workspace build completes in <5 minutes
- ✅ CLI provides complete orchestrator management capabilities
- ✅ Development tools integrate seamlessly with v2 architecture
- ✅ CI/CD pipeline supports all development workflows

## Component Analysis

### GitClone Development Components to Integrate

#### 1. Development Setup Scripts (`tools/dev-setup`)
```bash
# Current GitClone setup infrastructure
#!/bin/bash
# dev-setup/setup.sh

setup_rust_environment() {
    # Install Rust toolchain
    # Configure cargo settings
    # Install development tools
}

setup_nas_simulation() {
    # Setup mock NAS environment
    # Configure test data
    # Initialize development databases
}

setup_mcp_testing() {
    # Setup MCP test cluster
    # Configure protocol testing
    # Initialize federation testing
}
```

#### 2. Workspace Configuration (`Cargo.toml`, `.cargo/config.toml`)
```toml
# GitClone workspace configuration
[workspace]
members = [
    "crates/nestgate-agent",
    "crates/nestgate-core", 
    "crates/nestgate-protocol",
    "crates/nestgate-cli",
]

[workspace.dependencies]
# Well-organized dependency management
tokio = { version = "1.44.0", features = ["full"] }
serde = { version = "1.0.219", features = ["derive"] }
tracing = "0.1.41"
```

#### 3. CLI Implementation (`crates/nestgate-cli`)
```rust
// GitClone CLI structure
pub struct NestGateCli {
    config: CliConfig,
    client: ApiClient,
}

impl NestGateCli {
    pub async fn execute_command(&self, command: Command) -> Result<(), CliError> {
        match command {
            Command::Storage(storage_cmd) => self.handle_storage_command(storage_cmd).await,
            Command::Network(network_cmd) => self.handle_network_command(network_cmd).await,
            Command::System(system_cmd) => self.handle_system_command(system_cmd).await,
        }
    }
}
```

## v2 Development Tools Architecture

### Enhanced Workspace Configuration
```toml
# Cargo.toml (Enhanced for v2 orchestrator)
[workspace]
members = [
    # Core v2 components
    "code/crates/nestgate-orchestrator",
    "code/crates/nestgate-core", 
    "code/crates/nestgate-network",
    "code/crates/nestgate-zfs",
    "code/crates/nestgate-meta",
    
    # Integrated from GitClone
    "code/crates/nestgate-protocol",
    "code/crates/nestgate-cli",
    
    # Testing infrastructure
    "tests/mock",
    "tests/scenarios",
    "tests/performance",
    
    # Development tools
    "tools/dev-setup",
    "tools/mock-services",
]

[workspace.dependencies]
# Core runtime (from GitClone)
tokio = { version = "1.44.0", features = ["full"] }
futures = "0.3"
async-trait = "0.1"

# Logging and tracing (from GitClone)
tracing = { version = "0.1.41", features = ["default"] }
tracing-subscriber = { version = "0.3.19", features = ["env-filter", "json"] }
tracing-opentelemetry = "0.22"

# Serialization (from GitClone)
serde = { version = "1.0.219", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0.140"

# Network and HTTP (from GitClone)
axum = { version = "0.7", features = ["headers", "tracing"] }
tower = { version = "0.4.13", features = ["util", "timeout", "limit"] }
tower-http = { version = "0.4.4", features = ["trace", "cors"] }

# gRPC and Protocol Buffers (from GitClone)
tonic = { version = "0.10", features = ["tls", "compression"] }
prost = "0.12"
prost-build = "0.12"

# Storage and ZFS (from GitClone)
libzfs = "0.8"
notify = "6.1"

# Security (from GitClone)
ring = "0.17"
argon2 = "0.5"
jwt = "0.16"

# Testing (from GitClone)
criterion = { version = "0.5", features = ["html_reports"] }
mockall = "0.12"
proptest = "1.4"
test-case = "3.3"

# CLI tools
clap = { version = "4.0", features = ["derive", "env"] }
clap_complete = "4.0"
indicatif = "0.17"
console = "0.15"

# Error handling
thiserror = "2.0.12"
anyhow = "1.0"

# Utilities
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
humantime = "2.1"

[workspace.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

[profile.dev]
debug = true
opt-level = 0

[profile.release]
debug = false
opt-level = 3
lto = true
codegen-units = 1

[profile.test]
debug = true
opt-level = 1
```

### v2 Orchestrator CLI
```rust
// code/crates/nestgate-cli/src/main.rs
use clap::{Parser, Subcommand};
use nestgate_orchestrator_client::OrchestratorClient;

#[derive(Parser)]
#[command(name = "nestgate")]
#[command(about = "NestGate v2 Orchestrator CLI")]
#[command(version = "2.0.0")]
pub struct Cli {
    /// Orchestrator endpoint
    #[arg(long, env = "NESTGATE_ORCHESTRATOR_URL", default_value = "http://localhost:8080")]
    pub orchestrator_url: String,
    
    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub output: OutputFormat,
    
    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Orchestrator management commands
    Orchestrator {
        #[command(subcommand)]
        command: OrchestratorCommands,
    },
    /// Service management commands
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },
    /// Storage management commands
    Storage {
        #[command(subcommand)]
        command: StorageCommands,
    },
    /// Network management commands
    Network {
        #[command(subcommand)]
        command: NetworkCommands,
    },
    /// Federation management commands
    Federation {
        #[command(subcommand)]
        command: FederationCommands,
    },
    /// System health and monitoring
    Health {
        #[command(subcommand)]
        command: HealthCommands,
    },
}

#[derive(Subcommand)]
pub enum OrchestratorCommands {
    /// Show orchestrator status
    Status,
    /// Show orchestrator configuration
    Config,
    /// Restart orchestrator
    Restart,
    /// Show orchestrator logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    /// List all registered services
    List,
    /// Show service details
    Show {
        /// Service name
        name: String,
    },
    /// Register a new service
    Register {
        /// Service name
        name: String,
        /// Service endpoint URL
        endpoint: String,
        /// Service type
        #[arg(value_enum)]
        service_type: ServiceType,
    },
    /// Deregister a service
    Deregister {
        /// Service name
        name: String,
    },
    /// Show service health
    Health {
        /// Service name (optional, shows all if not specified)
        name: Option<String>,
    },
    /// Restart a service
    Restart {
        /// Service name
        name: String,
    },
}

#[derive(Subcommand)]
pub enum StorageCommands {
    /// List storage pools
    Pools,
    /// Create a new storage pool
    CreatePool {
        /// Pool name
        name: String,
        /// Storage tier
        #[arg(value_enum)]
        tier: StorageTier,
        /// Pool configuration
        #[arg(long)]
        config: Option<String>,
    },
    /// Show pool details
    ShowPool {
        /// Pool name
        name: String,
    },
    /// List storage exports
    Exports,
    /// Create a new export
    CreateExport {
        /// Export name
        name: String,
        /// Pool name
        pool: String,
        /// Export protocol
        #[arg(value_enum)]
        protocol: ExportProtocol,
    },
}

#[derive(Subcommand)]
pub enum FederationCommands {
    /// Show federation status
    Status,
    /// Connect to MCP cluster
    Connect {
        /// MCP cluster endpoint
        endpoint: String,
    },
    /// Disconnect from MCP cluster
    Disconnect,
    /// List federation capabilities
    Capabilities,
    /// Show federation configuration
    Config,
    /// Set federation mode
    SetMode {
        /// Federation mode
        #[arg(value_enum)]
        mode: FederationMode,
    },
}

// CLI implementation
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize logging based on verbosity
    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info", 
        2 => "debug",
        _ => "trace",
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();
    
    // Create orchestrator client
    let client = OrchestratorClient::new(&cli.orchestrator_url).await?;
    
    // Execute command
    let executor = CommandExecutor::new(client, cli.output);
    executor.execute(cli.command).await?;
    
    Ok(())
}
```

### CLI Command Executor
```rust
// code/crates/nestgate-cli/src/executor.rs
pub struct CommandExecutor {
    client: OrchestratorClient,
    output_format: OutputFormat,
}

impl CommandExecutor {
    pub async fn execute(&self, command: Commands) -> Result<(), CliError> {
        match command {
            Commands::Orchestrator { command } => self.execute_orchestrator_command(command).await,
            Commands::Service { command } => self.execute_service_command(command).await,
            Commands::Storage { command } => self.execute_storage_command(command).await,
            Commands::Network { command } => self.execute_network_command(command).await,
            Commands::Federation { command } => self.execute_federation_command(command).await,
            Commands::Health { command } => self.execute_health_command(command).await,
        }
    }
    
    async fn execute_orchestrator_command(&self, command: OrchestratorCommands) -> Result<(), CliError> {
        match command {
            OrchestratorCommands::Status => {
                let status = self.client.get_orchestrator_status().await?;
                self.output_orchestrator_status(&status);
            }
            OrchestratorCommands::Config => {
                let config = self.client.get_orchestrator_config().await?;
                self.output_orchestrator_config(&config);
            }
            OrchestratorCommands::Restart => {
                println!("Restarting orchestrator...");
                self.client.restart_orchestrator().await?;
                println!("✅ Orchestrator restarted successfully");
            }
            OrchestratorCommands::Logs { lines, follow } => {
                if follow {
                    self.follow_orchestrator_logs().await?;
                } else {
                    let logs = self.client.get_orchestrator_logs(lines).await?;
                    self.output_logs(&logs);
                }
            }
        }
        Ok(())
    }
    
    async fn execute_service_command(&self, command: ServiceCommands) -> Result<(), CliError> {
        match command {
            ServiceCommands::List => {
                let services = self.client.list_services().await?;
                self.output_services_table(&services);
            }
            ServiceCommands::Show { name } => {
                let service = self.client.get_service(&name).await?;
                self.output_service_details(&service);
            }
            ServiceCommands::Register { name, endpoint, service_type } => {
                let registration = ServiceRegistration {
                    name: name.clone(),
                    endpoint,
                    service_type,
                };
                self.client.register_service(registration).await?;
                println!("✅ Service '{}' registered successfully", name);
            }
            ServiceCommands::Health { name } => {
                match name {
                    Some(service_name) => {
                        let health = self.client.get_service_health(&service_name).await?;
                        self.output_service_health(&service_name, &health);
                    }
                    None => {
                        let all_health = self.client.get_all_service_health().await?;
                        self.output_all_service_health(&all_health);
                    }
                }
            }
        }
        Ok(())
    }
    
    async fn execute_federation_command(&self, command: FederationCommands) -> Result<(), CliError> {
        match command {
            FederationCommands::Status => {
                let status = self.client.get_federation_status().await?;
                self.output_federation_status(&status);
            }
            FederationCommands::Connect { endpoint } => {
                println!("Connecting to MCP cluster at {}...", endpoint);
                self.client.connect_federation(&endpoint).await?;
                println!("✅ Connected to MCP cluster successfully");
            }
            FederationCommands::Disconnect => {
                println!("Disconnecting from MCP cluster...");
                self.client.disconnect_federation().await?;
                println!("✅ Disconnected from MCP cluster");
            }
            FederationCommands::Capabilities => {
                let capabilities = self.client.get_federation_capabilities().await?;
                self.output_federation_capabilities(&capabilities);
            }
            FederationCommands::SetMode { mode } => {
                self.client.set_federation_mode(mode).await?;
                println!("✅ Federation mode set to {:?}", mode);
            }
        }
        Ok(())
    }
}
```

### Development Environment Setup
```bash
#!/bin/bash
# tools/dev-setup/v2-setup.sh (Enhanced from GitClone)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check system requirements
check_requirements() {
    log_info "Checking system requirements..."
    
    # Check Rust installation
    if ! command -v rustc &> /dev/null; then
        log_error "Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # Check Rust version
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    log_info "Found Rust version: $RUST_VERSION"
    
    # Check Cargo
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed"
        exit 1
    fi
    
    # Check Protocol Buffers compiler
    if ! command -v protoc &> /dev/null; then
        log_warning "protoc not found, installing..."
        install_protoc
    fi
    
    log_success "System requirements satisfied"
}

# Install Protocol Buffers compiler
install_protoc() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        sudo apt-get update
        sudo apt-get install -y protobuf-compiler libprotobuf-dev
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install protobuf
        else
            log_error "Homebrew not found. Please install protobuf manually."
            exit 1
        fi
    else
        log_error "Unsupported OS for automatic protoc installation"
        exit 1
    fi
}

# Install Rust development tools
install_rust_tools() {
    log_info "Installing Rust development tools..."
    
    # Core development tools
    cargo install cargo-watch --quiet
    cargo install cargo-nextest --quiet
    cargo install cargo-audit --quiet
    cargo install cargo-deny --quiet
    cargo install cargo-machete --quiet
    
    # Documentation tools
    cargo install mdbook --quiet
    cargo install mdbook-mermaid --quiet
    
    # Formatting and linting
    rustup component add rustfmt
    rustup component add clippy
    
    log_success "Rust development tools installed"
}

# Setup workspace configuration
setup_workspace() {
    log_info "Setting up workspace configuration..."
    
    # Create .cargo directory if it doesn't exist
    mkdir -p .cargo
    
    # Create cargo config
    cat > .cargo/config.toml << 'EOF'
[build]
rustflags = ["-D", "warnings"]

[env]
RUST_BACKTRACE = "1"

[alias]
# Development aliases
dev = "watch -x 'check --all'"
test-all = "nextest run --all"
lint = "clippy --all-targets --all-features -- -D warnings"
fmt-check = "fmt --all -- --check"
audit-all = "audit --deny warnings"

# Build aliases
build-release = "build --release --all"
build-debug = "build --all"

# Documentation aliases
doc-open = "doc --open --no-deps"
doc-all = "doc --all --no-deps"
EOF

    # Create rustfmt configuration
    cat > rustfmt.toml << 'EOF'
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
edition = "2021"
EOF

    # Create clippy configuration
    cat > .clippy.toml << 'EOF'
msrv = "1.70"
avoid-breaking-exported-api = false
EOF

    log_success "Workspace configuration created"
}

# Setup mock services for testing
setup_mock_services() {
    log_info "Setting up mock services..."
    
    # Create test data directories
    mkdir -p test-data/{hot,warm,cold}
    mkdir -p test-data/exports
    mkdir -p test-data/metadata
    
    # Create mock ZFS pools
    if [[ -f "tests/mock/setup-mock-zfs.sh" ]]; then
        chmod +x tests/mock/setup-mock-zfs.sh
        ./tests/mock/setup-mock-zfs.sh
    fi
    
    # Build mock services
    log_info "Building mock services..."
    cargo build --package tests-mock --release
    
    log_success "Mock services setup complete"
}

# Setup development database
setup_dev_database() {
    log_info "Setting up development database..."
    
    # Create metadata storage directories
    mkdir -p dev-data/metadata
    mkdir -p dev-data/config
    mkdir -p dev-data/logs
    
    # Initialize development configuration
    cat > dev-data/config/orchestrator.yaml << 'EOF'
orchestrator:
  bind_address: "0.0.0.0:8080"
  log_level: "debug"
  
  service_registry:
    enable_discovery: true
    registration_timeout: 30s
    
  health_monitoring:
    check_interval: 10s
    failure_threshold: 3
    restart_enabled: true
    
  mcp_integration:
    enabled: true
    mode: "auto_detect"
    
  federation:
    mode: "auto_detect"
    discovery_interval: 300s
    heartbeat_interval: 30s
EOF
    
    log_success "Development database setup complete"
}

# Setup IDE configuration
setup_ide_config() {
    log_info "Setting up IDE configuration..."
    
    # VS Code settings
    mkdir -p .vscode
    cat > .vscode/settings.json << 'EOF'
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.allTargets": true,
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "files.watcherExclude": {
        "**/target/**": true
    },
    "search.exclude": {
        "**/target": true,
        "**/Cargo.lock": true
    }
}
EOF

    cat > .vscode/extensions.json << 'EOF'
{
    "recommendations": [
        "rust-lang.rust-analyzer",
        "tamasfe.even-better-toml",
        "serayuzgur.crates",
        "vadimcn.vscode-lldb"
    ]
}
EOF

    # Create launch configuration
    cat > .vscode/launch.json << 'EOF'
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Orchestrator",
            "cargo": {
                "args": [
                    "build",
                    "--bin=nestgate-orchestrator",
                    "--package=nestgate-orchestrator"
                ],
                "filter": {
                    "name": "nestgate-orchestrator",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_LOG": "debug"
            }
        }
    ]
}
EOF

    log_success "IDE configuration created"
}

# Run initial tests
run_initial_tests() {
    log_info "Running initial tests..."
    
    # Check that everything compiles
    log_info "Checking compilation..."
    cargo check --all
    
    # Run quick tests
    log_info "Running unit tests..."
    cargo test --lib --all
    
    # Run clippy
    log_info "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    # Check formatting
    log_info "Checking formatting..."
    cargo fmt --all -- --check
    
    log_success "Initial tests passed"
}

# Main setup function
main() {
    log_info "🚀 Setting up NestGate v2 development environment..."
    
    check_requirements
    install_rust_tools
    setup_workspace
    setup_mock_services
    setup_dev_database
    setup_ide_config
    run_initial_tests
    
    log_success "✅ NestGate v2 development environment ready!"
    echo ""
    log_info "Quick start commands:"
    echo "  cargo run --bin nestgate-orchestrator    # Start the orchestrator"
    echo "  cargo test --all                         # Run all tests"
    echo "  cargo dev                                # Watch for changes"
    echo "  ./tools/dev-setup/start-dev-env.sh       # Start full dev environment"
    echo ""
    log_info "For more information, see docs/development/README.md"
}

# Run main function
main "$@"
```

### Development Environment Management
```bash
#!/bin/bash
# tools/dev-setup/start-dev-env.sh

set -euo pipefail

# Start development environment with all services
start_dev_environment() {
    echo "🚀 Starting NestGate v2 development environment..."
    
    # Start orchestrator in background
    echo "Starting orchestrator..."
    cargo run --bin nestgate-orchestrator &
    ORCHESTRATOR_PID=$!
    
    # Wait for orchestrator to start
    sleep 5
    
    # Start mock services
    echo "Starting mock services..."
    cargo run --bin mock-nestgate-core &
    CORE_PID=$!
    
    cargo run --bin mock-nestgate-network &
    NETWORK_PID=$!
    
    cargo run --bin mock-nestgate-zfs &
    ZFS_PID=$!
    
    # Wait for services to register
    sleep 3
    
    # Show status
    echo "✅ Development environment started!"
    echo ""
    echo "Services:"
    echo "  Orchestrator: http://localhost:8080"
    echo "  Mock Core:    http://localhost:8081"
    echo "  Mock Network: http://localhost:8082"
    echo "  Mock ZFS:     http://localhost:8083"
    echo ""
    echo "Try: cargo run --bin nestgate -- orchestrator status"
    echo ""
    echo "Press Ctrl+C to stop all services"
    
    # Setup signal handlers
    trap 'kill $ORCHESTRATOR_PID $CORE_PID $NETWORK_PID $ZFS_PID 2>/dev/null; exit' INT TERM
    
    # Wait for interrupt
    wait
}

start_dev_environment
```

## Integration Implementation Plan

### Phase 1: Workspace Enhancement (Week 1)
```yaml
tasks:
  day_1_2:
    - Integrate GitClone Cargo.toml configuration
    - Merge dependency management
    - Setup enhanced workspace structure
    - Resolve dependency conflicts
  
  day_3_4:
    - Integrate development setup scripts
    - Adapt scripts for v2 orchestrator
    - Create IDE configuration
    - Setup development tools
  
  day_5:
    - Test full workspace build
    - Validate development environment
    - Performance baseline measurements
```

### Phase 2: CLI Integration (Week 2)
```yaml
tasks:
  day_1_2:
    - Adapt GitClone CLI for v2 orchestrator
    - Implement orchestrator client
    - Create command structure
    - Basic CLI functionality
  
  day_3_4:
    - Implement all CLI commands
    - Add output formatting
    - Create interactive features
    - CLI testing and validation
  
  day_5:
    - End-to-end CLI testing
    - Documentation and help system
    - Shell completion setup
```

### Phase 3: CI/CD Integration (Week 3)
```yaml
tasks:
  day_1_2:
    - Adapt GitClone CI/CD workflows
    - Create GitHub Actions for v2
    - Setup automated testing
    - Performance monitoring
  
  day_3_4:
    - Advanced CI/CD features
    - Deployment automation
    - Security scanning
    - Documentation generation
  
  day_5:
    - CI/CD validation
    - Performance optimization
    - Monitoring setup
```

## CI/CD Pipeline Integration

### GitHub Actions Workflow
```yaml
# .github/workflows/ci.yml
name: NestGate v2 CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check and Lint
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Install protoc
      run: sudo apt-get update && sudo apt-get install -y protobuf-compiler
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Check compilation
      run: cargo check --all

  test:
    name: Test Suite
    runs-on: ubuntu-latest
    needs: check
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install protoc
      run: sudo apt-get update && sudo apt-get install -y protobuf-compiler
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Install nextest
      run: cargo install cargo-nextest --locked
    
    - name: Run unit tests
      run: cargo nextest run --lib --all
    
    - name: Run integration tests
      run: cargo nextest run --test '*' --all
    
    - name: Run doc tests
      run: cargo test --doc --all

  performance:
    name: Performance Tests
    runs-on: ubuntu-latest
    needs: check
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install protoc
      run: sudo apt-get update && sudo apt-get install -y protobuf-compiler
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run benchmarks
      run: cargo bench --all
    
    - name: Upload benchmark results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: target/criterion/

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install cargo-audit
      run: cargo install cargo-audit --locked
    
    - name: Run security audit
      run: cargo audit
    
    - name: Install cargo-deny
      run: cargo install cargo-deny --locked
    
    - name: Run dependency check
      run: cargo deny check

  build:
    name: Build Release
    runs-on: ubuntu-latest
    needs: [test, performance, security]
    if: github.ref == 'refs/heads/main'
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install protoc
      run: sudo apt-get update && sudo apt-get install -y protobuf-compiler
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Build release
      run: cargo build --release --all
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: nestgate-v2-binaries
        path: |
          target/release/nestgate-orchestrator
          target/release/nestgate
```

## Developer Experience Enhancements

### Quick Start Scripts
```bash
#!/bin/bash
# tools/dev-setup/quick-start.sh

# One-command development environment setup
curl -sSL https://raw.githubusercontent.com/nestgate/nestgate/main/tools/dev-setup/v2-setup.sh | bash

# Start development environment
./tools/dev-setup/start-dev-env.sh
```

### Development Aliases
```bash
# tools/dev-setup/dev-aliases.sh
# Source this file to get helpful development aliases

alias ng='cargo run --bin nestgate --'
alias ngo='cargo run --bin nestgate-orchestrator'
alias ngt='cargo nextest run --all'
alias ngb='cargo build --all'
alias ngc='cargo check --all'
alias ngl='cargo clippy --all-targets --all-features -- -D warnings'
alias ngf='cargo fmt --all'
alias ngd='cargo doc --open --no-deps'

# Development workflow aliases
alias ng-dev='cargo watch -x "check --all"'
alias ng-test='cargo watch -x "nextest run --all"'
alias ng-full='cargo check --all && cargo test --all && cargo clippy --all-targets --all-features -- -D warnings'

echo "NestGate v2 development aliases loaded!"
echo "Use 'ng --help' for CLI help"
echo "Use 'ngo' to start orchestrator"
echo "Use 'ngt' to run tests"
```

## Performance Optimizations

### Build Performance
```toml
# .cargo/config.toml (Performance optimizations)
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Faster builds for development
[profile.dev]
debug = 1
incremental = true
```

### Development Metrics
```yaml
development_metrics:
  setup_time:
    target: "<10 minutes"
    measurement: "Time from git clone to running orchestrator"
    
  build_time:
    target: "<5 minutes"
    measurement: "Full workspace build time"
    
  test_time:
    target: "<3 minutes"
    measurement: "Full test suite execution"
    
  cli_responsiveness:
    target: "<1 second"
    measurement: "CLI command response time"
```

## Summary

The development tools integration will significantly enhance the NestGate v2 development experience:

### Key Benefits
1. **Streamlined Setup**: One-command environment setup in <10 minutes
2. **Enhanced CLI**: Complete orchestrator management through CLI
3. **Improved Workflow**: Efficient development, testing, and deployment
4. **Better Tooling**: Advanced IDE configuration and development tools

### Integration Achievements
- ✅ **Enhanced Workspace**: Sophisticated dependency management and build configuration
- ✅ **v2 CLI**: Complete CLI for orchestrator-centric operations
- ✅ **Development Environment**: Automated setup and management scripts
- ✅ **CI/CD Pipeline**: Comprehensive testing and deployment automation

The integration ensures that developers can be productive immediately with NestGate v2, backed by mature development tools and streamlined workflows. 