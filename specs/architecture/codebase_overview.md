---
title: NestGate Codebase Layout
description: Detailed structure and organization of the NestGate project
version: 0.1.0
---

# NestGate Codebase Layout

## Project Structure

```
nestgate/
├── .cargo/                    # Cargo configuration
│   └── config.toml           # Cargo settings
├── .cursor/                   # Cursor IDE configuration
│   └── rules/                # Development rules and standards
├── .github/                   # GitHub configuration
│   ├── workflows/            # GitHub Actions CI/CD
│   └── CODEOWNERS            # Code ownership definitions
├── specs/                     # System specifications
│   ├── core/                 # Core system specifications
│   ├── dev/                  # Development specifications
│   ├── network/              # Network specifications
│   └── storage/              # Storage specifications
├── crates/                    # Rust crates
│   ├── nestgate-agent/       # NAS management agent
│   │   ├── src/             # Source code
│   │   ├── tests/           # Tests
│   │   └── Cargo.toml       # Crate manifest
│   ├── nestgate-core/        # Core functionality
│   │   ├── src/             # Source code
│   │   ├── tests/           # Tests
│   │   └── Cargo.toml       # Crate manifest
│   ├── nestgate-protocol/    # MCP protocol implementation
│   │   ├── src/             # Source code
│   │   ├── tests/           # Tests
│   │   └── Cargo.toml       # Crate manifest
│   └── nestgate-cli/         # Command-line interface
│       ├── src/             # Source code
│       ├── tests/           # Tests
│       └── Cargo.toml       # Crate manifest
├── tools/                    # Development and maintenance tools
│   ├── dev-setup/           # Development environment setup
│   ├── mock-nas/            # NAS simulation for testing
│   └── performance/         # Performance testing tools
├── docs/                     # Documentation
│   ├── api/                 # API documentation
│   ├── architecture/        # Architecture documentation
│   ├── deployment/         # Deployment guides
│   └── development/        # Development guides
├── tests/                    # Integration and system tests
│   ├── integration/        # Integration test suites
│   ├── performance/        # Performance test suites
│   └── security/          # Security test suites
├── .dev-tools/              # Development and testing tools
│   ├── docker/            # Docker configurations
│   └── ... other tools    # Other development tools
├── .gitignore              # Git ignore patterns
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── CODEBASE.md            # This file
├── README.md              # Project overview
└── rustfmt.toml           # Rust formatting configuration
```

## Component Details

### Core Crates

#### nestgate-agent
- Primary NAS management agent implementation
- Handles communication with NAS systems
- Implements state management and monitoring
- Directory structure:
  ```
  nestgate-agent/
  ├── src/
  │   ├── commands/        # Command implementations
  │   ├── state/          # State management
  │   ├── monitoring/     # Monitoring and metrics
  │   └── protocols/      # Protocol implementations
  ├── tests/
  │   ├── integration/    # Integration tests
  │   └── unit/          # Unit tests
  └── examples/           # Usage examples
  ```

#### nestgate-core
- Core functionality and shared components
- Common utilities and interfaces
- Directory structure:
  ```
  nestgate-core/
  ├── src/
  │   ├── error/          # Error definitions
  │   ├── config/         # Configuration management
  │   ├── utils/          # Shared utilities
  │   └── types/          # Common type definitions
  └── tests/              # Test suites
  ```

#### nestgate-protocol
- MCP (Machine Context Protocol) implementation
- Protocol definitions and handlers
- Directory structure:
  ```
  nestgate-protocol/
  ├── src/
  │   ├── codec/          # Protocol encoding/decoding
  │   ├── messages/       # Message definitions
  │   ├── handlers/       # Message handlers
  │   └── validation/     # Protocol validation
  └── tests/              # Test suites
  ```

#### nestgate-cli
- Command-line interface implementation
- User interaction and command processing
- Directory structure:
  ```
  nestgate-cli/
  ├── src/
  │   ├── commands/       # CLI commands
  │   ├── output/         # Output formatting
  │   ├── config/         # CLI configuration
  │   └── interactive/    # Interactive mode
  └── tests/              # Test suites
  ```

### Development Tools

#### dev-setup
- Development environment configuration
- Local testing infrastructure
- Directory structure:
  ```
  tools/dev-setup/
  ├── scripts/            # Setup scripts
  ├── docker/            # Docker configurations
  └── k3d/               # Local Kubernetes setup
  ```

#### mock-nas
- NAS system simulation for testing
- Test scenario implementations
- Directory structure:
  ```
  tools/mock-nas/
  ├── src/               # Mock implementation
  ├── scenarios/         # Test scenarios
  └── data/             # Test data
  ```

### Documentation

#### Architecture Documentation
- System design and architecture
- Component interactions
- Directory structure:
  ```
  docs/architecture/
  ├── overview/          # System overview
  ├── components/        # Component details
  ├── protocols/         # Protocol specifications
  └── decisions/         # Architecture decisions
  ```

#### Development Documentation
- Development guides and standards
- Directory structure:
  ```
  docs/development/
  ├── setup/             # Setup guides
  ├── workflow/          # Development workflow
  ├── testing/           # Testing guides
  └── style/             # Style guides
  ```

## Development Guidelines

### Adding New Components
1. Create new crate in `crates/` directory
2. Update workspace `Cargo.toml`
3. Add component documentation
4. Update integration tests

### Documentation Updates
1. Update relevant markdown files
2. Keep directory structure documentation current
3. Update cross-references
4. Maintain API documentation

### Testing Requirements
1. Unit tests with each component
2. Integration tests in `tests/` directory
3. Performance tests where applicable
4. Security testing for sensitive components

## Version Control

### Branch Structure
- `main`: Primary development branch
- `feature/*`: Feature development
- `release/*`: Release preparation
- `hotfix/*`: Production fixes

### Commit Guidelines
- Follow conventional commits
- Reference issues and specifications
- Include component scope
- Keep changes focused 