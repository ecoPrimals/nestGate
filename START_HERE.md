# 🦅 Welcome to NestGate!

**Last Updated**: January 14, 2026

---

## 👋 **New to NestGate?**

Welcome! This guide will get you started quickly.

---

## 🎯 **Quick Overview**

**NestGate** is a sovereign, zero-cost storage orchestrator for the Primal Ecosystem.

**Current Status**: **B+ (88/100)** - Production capable with ongoing evolution

**Key Features**:
- 🤖 Infant Discovery Architecture
- ⚡ Zero-Cost Performance
- 🌐 Universal Storage Adapter
- 🔒 Sovereign by Design
- 🛡️ Top 0.1% Safety

---

## 🚀 **5-Minute Quick Start**

### **1. Clone and Build**:

```bash
git clone https://github.com/your-org/nestgate.git
cd nestgate
cargo build --release
```

### **2. Run Tests**:

```bash
cargo test
# Expected: 3,607 tests passing ✅
```

### **3. Start NestGate**:

```bash
./start_local_dev.sh
```

### **4. Check Status**:

```bash
curl http://localhost:8000/health
```

**Done!** 🎉 NestGate is running!

---

## 📚 **What to Read Next**

### **For Users**:
1. 📖 **[README](README.md)** - Project overview and features
2. 🎯 **[Current Status](CURRENT_STATUS.md)** - Project health and metrics
3. 📝 **[Quick Reference](QUICK_REFERENCE.md)** - Common commands and patterns
4. 🗺️ **[Roadmap](ROADMAP.md)** - Future plans and milestones

### **For Developers**:
1. 🤝 **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
2. 🏗️ **[Architecture Docs](docs/architecture/)** - System design
3. 🧪 **[Testing Guide](docs/testing/)** - Writing tests
4. 📊 **[Latest Session Report](EXCEPTIONAL_SESSION_COMPLETE_JAN_13_2026.md)** - Recent progress

### **For Operators**:
1. 🚀 **[Deployment Guide](docs/operations/)** - Production deployment
2. 📊 **[Monitoring Guide](docs/operations/)** - Observability setup
3. 🔧 **[Troubleshooting](docs/guides/)** - Common issues
4. 🔐 **[Security Guide](docs/security/)** - Security best practices

---

## 🏗️ **Architecture Overview**

### **Core Components**:

```
NestGate
├── nestgate-core         - Orchestration engine
│   ├── Infant Discovery  - Runtime primal discovery
│   ├── Zero-Cost Layer   - Performance optimizations
│   └── Universal Adapter - Storage abstraction
│
├── nestgate-api          - REST API
│   ├── Handlers          - Request handlers
│   └── Middleware        - Auth, logging, etc.
│
├── nestgate-zfs          - ZFS Backend
│   ├── Pool Management   - ZFS pool operations
│   ├── Snapshots         - Snapshot management
│   └── Monitoring        - Health checks
│
├── nestgate-mcp          - MCP Protocol
│   ├── Messages          - Protocol messages
│   └── Handlers          - Message processing
│
└── nestgate-performance  - Optimizations
    ├── Zero-Copy         - Network optimizations
    ├── Memory            - Memory management
    └── SIMD              - Vector operations
```

### **Key Concepts**:

1. **Infant Discovery**: Primals discover each other at runtime
2. **Zero-Cost**: Native performance with high-level APIs
3. **Universal Adapter**: Works with any storage backend
4. **Capability-Based**: Dynamic service routing
5. **Sovereign**: No vendor lock-in, full control

---

## 🧪 **Testing**

### **Run All Tests**:

```bash
cargo test
# Expected: 3,607 passing ✅
```

### **Run Specific Tests**:

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific module
cargo test --package nestgate-core

# With output
cargo test -- --nocapture
```

### **Coverage Report**:

```bash
cargo llvm-cov --html
# Open: target/llvm-cov/html/index.html
```

---

## 📊 **Current Status**

### **Health Check**:

```
✅ Tests: 3,607 passing (100%)
✅ Build: Compiling successfully
✅ Lints: Passing (5 minor warnings)
⚠️ Coverage: 70% (target: 90%)
```

### **Grade**: **B+ (88/100)**

```
Architecture:     A+ (98/100) ✅ Excellent
Sovereignty:      A+ (100/100) ✅ Perfect
Safety:           A  (93/100) ✅ Excellent
Test Coverage:    C+ (78/100) ⚠️ Needs work
Error Handling:   D+ (65/100) ❌ Critical
```

**Path to A+**: 8-week systematic evolution plan

📊 **[View Full Status](CURRENT_STATUS.md)**

---

## 🎯 **Common Tasks**

### **Development**:

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Documentation
cargo doc --open
```

### **Deployment**:

```bash
# Build release
cargo build --release

# Run production
./QUICK_DEPLOY.sh

# Check deployment
./verify_deployment_readiness.sh
```

---

## 🔧 **Configuration**

### **Local Development**:

```bash
# Copy example config
cp config/production.env.example .env

# Edit configuration
nano .env

# Start with config
./start_local_dev.sh
```

### **Production**:

See [Deployment Guide](docs/operations/) for production configuration.

---

## 🐛 **Troubleshooting**

### **Build Issues**:

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

### **Test Failures**:

```bash
# Run failing test with output
cargo test <test_name> -- --nocapture

# Run specific package
cargo test --package nestgate-core
```

### **Common Issues**:

- **Port already in use**: Check `config/production.env` for port settings
- **ZFS not available**: Install ZFS or use mock backends for testing
- **Permission denied**: Run with appropriate permissions or use Docker

See [Troubleshooting Guide](docs/guides/) for more.

---

## 📖 **Learning Resources**

### **Documentation**:
- 📁 [docs/](docs/) - Full documentation
- 📊 [specs/](specs/) - Specifications
- 🎯 [examples/](examples/) - Code examples

### **Recent Progress**:
- 📝 [Latest Session Report](EXCEPTIONAL_SESSION_COMPLETE_JAN_13_2026.md)
- 📋 [Session Reports](docs/session-reports/2026-01-jan/)

### **Community**:
- 💬 [GitHub Discussions](https://github.com/your-org/nestgate/discussions)
- 🐛 [Issue Tracker](https://github.com/your-org/nestgate/issues)
- 📧 [Email Support](mailto:support@your-org.com)

---

## 🤝 **Contributing**

We welcome contributions! Here's how to get started:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Fork the repository
3. Create a feature branch
4. Make your changes
5. Run tests (`cargo test`)
6. Submit a Pull Request

---

## 📞 **Getting Help**

### **Stuck?**

1. Check [README.md](README.md) for overview
2. Check [CURRENT_STATUS.md](CURRENT_STATUS.md) for project health
3. Check [docs/](docs/) for detailed documentation
4. Check [GitHub Issues](https://github.com/your-org/nestgate/issues)
5. Ask in [GitHub Discussions](https://github.com/your-org/nestgate/discussions)

### **Found a Bug?**

1. Check [existing issues](https://github.com/your-org/nestgate/issues)
2. Create a new issue with:
   - Description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - System information

---

## 🎊 **Ready to Dive In?**

Great! Here are your next steps:

### **For Users**:
→ **[README.md](README.md)** - Learn about features  
→ **[Quick Reference](QUICK_REFERENCE.md)** - Common patterns  
→ **[Deployment Guide](docs/operations/)** - Production setup

### **For Developers**:
→ **[Contributing Guide](CONTRIBUTING.md)** - Start contributing  
→ **[Architecture Docs](docs/architecture/)** - Understand the system  
→ **[Latest Report](EXCEPTIONAL_SESSION_COMPLETE_JAN_13_2026.md)** - Recent progress

### **For Operators**:
→ **[Deployment Guide](docs/operations/)** - Production deployment  
→ **[Monitoring Guide](docs/operations/)** - Setup observability  
→ **[Security Guide](docs/security/)** - Security best practices

---

**Welcome to the NestGate family! 🦅**

**Questions?** → [GitHub Discussions](https://github.com/your-org/nestgate/discussions)  
**Issues?** → [Issue Tracker](https://github.com/your-org/nestgate/issues)  
**Ideas?** → We'd love to hear them!

---

**Happy Building! 🚀**
