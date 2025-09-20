# 🤝 Contributing to NestGate

Thank you for your interest in contributing to NestGate! We welcome contributions from developers of all skill levels and backgrounds.

## 🎯 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to providing a welcoming and inclusive environment for all contributors.

## 🚀 Getting Started

### Prerequisites

- **Rust 1.75+** with the latest stable toolchain
- **Git** for version control
- **IDE/Editor** with Rust support (VS Code with rust-analyzer recommended)
- **Docker** (optional, for integration testing)

### Development Setup

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/nestgate.git
cd nestgate

# Install development dependencies
rustup component add rustfmt clippy
cargo install cargo-audit cargo-tarpaulin

# Verify setup
cargo check --workspace
cargo test --workspace
```

## 🔍 Code Quality Standards

NestGate maintains **pedantic code quality standards**. All contributions must meet these requirements:

### 1. Compilation

```bash
# Must compile without errors or warnings
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

### 2. Pedantic Linting

```bash
# Must pass all pedantic lints
cargo clippy --workspace -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings
```

### 3. Formatting

```bash
# Code must be formatted with rustfmt
cargo fmt --all --check
```

### 4. Testing

```bash
# Minimum 90% test coverage required
cargo test --workspace
cargo tarpaulin --workspace --out Html
```

### 5. Documentation

```bash
# All public APIs must be documented
cargo doc --workspace --no-deps --document-private-items
```

### 6. Security

```bash
# Must pass security audit
cargo audit
```

## 📝 Development Workflow

### 1. Issue Creation

Before starting work:

- Check existing issues and PRs
- Create a detailed issue describing the problem/feature
- Wait for maintainer feedback and approval
- Get assigned to the issue

### 2. Branch Strategy

```bash
# Create feature branch from main
git checkout main
git pull origin main
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 3. Development Process

1. **Write Tests First**: Follow TDD when possible
2. **Small Commits**: Make atomic, well-described commits
3. **Follow Conventions**: Use established patterns and naming
4. **Document Changes**: Update docs for user-facing changes

### 4. Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:
```
feat(api): add user authentication endpoint
fix(core): resolve memory leak in connection pool
docs(readme): update installation instructions
test(security): add integration tests for JWT validation
```

### 5. Pull Request Process

1. **Ensure Quality**: All checks must pass
2. **Update Documentation**: Include relevant doc updates
3. **Add Tests**: Ensure adequate test coverage
4. **Describe Changes**: Provide detailed PR description
5. **Link Issues**: Reference related issues

#### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (cargo fmt)
- [ ] Lints pass (cargo clippy)
- [ ] Documentation updated
- [ ] Security audit passes
```

## 🏗️ Architecture Guidelines

### Code Organization

```
nestgate/
├── code/crates/
│   ├── nestgate-core/          # Core functionality
│   ├── nestgate-canonical/     # Canonical types
│   └── nestgate-server/        # Server implementation
├── tests/                      # Integration tests
├── docs/                       # Documentation
└── examples/                   # Example code
```

### Design Principles

1. **Zero-Cost Abstractions**: Prefer compile-time over runtime costs
2. **Memory Safety**: Leverage Rust's ownership system
3. **Performance**: Optimize for speed and efficiency
4. **Modularity**: Keep components loosely coupled
5. **Testability**: Design for easy testing
6. **Documentation**: Code should be self-documenting

### Naming Conventions

- **Functions**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`

## 🧪 Testing Guidelines

### Test Categories

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test component interactions
3. **Performance Tests**: Benchmark critical paths
4. **Security Tests**: Validate security measures

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name_should_describe_behavior() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Test Requirements

- **Descriptive Names**: Test names should describe expected behavior
- **Comprehensive Coverage**: Test happy path, edge cases, and error conditions
- **Isolated**: Tests should not depend on each other
- **Fast**: Unit tests should run quickly
- **Deterministic**: Tests should produce consistent results

## 📚 Documentation Standards

### Code Documentation

```rust
/// Brief one-line description
///
/// Longer description explaining the purpose, behavior,
/// and any important details.
///
/// # Arguments
///
/// * `param1` - Description of parameter
/// * `param2` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Examples
///
/// ```
/// let result = function_name(param1, param2)?;
/// assert_eq!(result, expected);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType, Error> {
    // Implementation
}
```

### README Updates

When adding features:
- Update feature list
- Add configuration examples
- Update usage examples
- Document breaking changes

## 🔒 Security Guidelines

### Security Practices

1. **Input Validation**: Validate all external inputs
2. **Error Handling**: Don't leak sensitive information
3. **Dependency Management**: Keep dependencies updated
4. **Secrets Management**: Never commit secrets
5. **Audit Trail**: Log security-relevant events

### Reporting Security Issues

**DO NOT** create public issues for security vulnerabilities.

Instead:
1. Email security@nestgate.dev
2. Provide detailed description
3. Include steps to reproduce
4. Allow 90 days for response

## 🎖️ Recognition

Contributors are recognized in:
- **CONTRIBUTORS.md** - All contributors listed
- **Release Notes** - Major contributions highlighted
- **GitHub Releases** - Contributors thanked in releases

## 📞 Getting Help

- **GitHub Discussions** - General questions and ideas
- **GitHub Issues** - Bug reports and feature requests
- **Discord** - Real-time chat (invite in README)
- **Email** - maintainers@nestgate.dev

## 🏆 Contribution Types

We value all types of contributions:

- **Code**: Bug fixes, features, optimizations
- **Documentation**: Guides, examples, API docs
- **Testing**: Test cases, performance benchmarks
- **Design**: UI/UX improvements, architecture
- **Community**: Answering questions, mentoring
- **Infrastructure**: CI/CD, tooling improvements

## 📋 Checklist for Maintainers

When reviewing PRs:

- [ ] Code quality meets pedantic standards
- [ ] Tests are comprehensive and pass
- [ ] Documentation is updated
- [ ] Breaking changes are documented
- [ ] Security implications considered
- [ ] Performance impact assessed
- [ ] Backwards compatibility maintained

Thank you for contributing to NestGate! Your efforts help make this project better for everyone. 🚀 