# NestGate Project Organization Principles

## Overview

This document outlines the principles and reasoning behind the NestGate project organization. The goal of this reorganization was to create a clear, maintainable structure that:

1. Separates concerns between different types of content
2. Provides clear boundaries between implementation and documentation
3. Makes it easy to navigate and understand the codebase
4. Follows industry best practices for project organization

## Core Organizational Principles

### 1. Separation of Implementation and Documentation

- **Implementation code** belongs in the `crates/` directory
- **User documentation** belongs in the `docs/` directory
- **System specifications** belong in the `specs/` directory
- **Utility scripts** belong in the `scripts/` directory

This separation ensures that each type of content has a dedicated location, making it easier to find and manage.

### 2. Hierarchical Organization by Function

Within each top-level directory, content is further organized by function:

- **Implementation code** is organized by component/service
- **Documentation** is organized by audience and purpose
- **Specifications** are organized by aspect of the system
- **Scripts** are organized by type of operation

### 3. Clear Naming Conventions

- Directory names use kebab-case (e.g., `nestgate-ui`)
- Files use appropriate extensions based on their content (`.md`, `.ts`, etc.)
- Descriptive names are preferred over abbreviations
- Related files are grouped together in subdirectories

### 4. Minimized Duplication

- No redundant copies of files
- Configuration is centralized
- Common utilities are shared
- Documentation references rather than duplicates content

## Directory Structure Rationale

### Implementation Code in `crates/`

The `crates/` directory follows the Rust crates concept, even for non-Rust code. Each subdirectory represents a logical component of the system with a clear responsibility:

- **nestgate-ui**: Frontend React application and server components
- **nestgate-api**: API implementation
- **nestgate-core**: Core functionality shared across components
- **nestgate-middleware**: Middleware components for interoperability
- **nestgate-ai-mock**: Mock implementations for AI testing
- **nestgate-port-manager**: Management of service ports

This organization allows for:
- Clear component boundaries
- Independent development and testing
- Focused responsibility for each component
- Easier onboarding for new developers

### Documentation in `docs/`

User-focused documentation is organized by type in the `docs/` directory:

- **api/**: API documentation for integrators and developers
- **guides/**: User guides and tutorials for end users
- **images/**: Screenshots and diagrams for visual documentation
- **references/**: Reference documentation for detailed lookups

This organization makes it easy for different audiences to find the information they need.

### Specifications in `specs/`

System design and specifications are organized by aspect in the `specs/` directory:

- **architecture/**: High-level architecture descriptions
- **implementation/**: Detailed implementation specifications
- **project/**: Project management documents
- **integration/**: Integration specifications for external systems

This organization helps separate different levels of technical detail and different aspects of the system design.

### Scripts in `scripts/`

Utility scripts are organized by operation in the `scripts/` directory:

- **build/**: Scripts for building components
- **start/**: Scripts for starting services
- **test/**: Scripts for running tests
- **migration/**: Scripts for migrating data or code
- **util/**: General utility scripts

This organization makes it easy to find the right script for a particular task.

## Testing Organization

Tests are organized alongside the code they test:

- **Unit tests** are placed in the same directory as the code they test
- **Integration tests** are placed in dedicated `tests/` directories
- **System tests** are placed in the top-level `tests/` directory
- **Mock data** is placed in `__mocks__` directories

## Benefits of This Organization

1. **Improved Navigability**: Developers can quickly find code, documentation, or scripts.
2. **Clear Boundaries**: The system is divided into logical components with clear responsibilities.
3. **Reduced Cognitive Load**: Developers only need to focus on relevant parts of the system.
4. **Better Maintainability**: Changes are isolated to specific components.
5. **Easier Onboarding**: New developers can understand the system structure quickly.
6. **Scalability**: The structure accommodates growth without becoming unwieldy.

## Conclusion

The NestGate project organization is designed to support long-term maintainability and clarity. By following these principles, we ensure that the codebase remains organized, navigable, and scalable as the project grows. 