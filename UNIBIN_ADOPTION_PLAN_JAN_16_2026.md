# UniBin Adoption Plan for NestGate - January 16, 2026

**Date**: January 16, 2026 (3:30 AM)  
**Source**: ToadStool uniBin evolution (FIRST uniBin primal!)  
**Goal**: Evolve NestGate to unified binary standard  
**Status**: ✅ **READY TO PLAN**

---

## 🎯 **What is UniBin?**

### **Definition**

**UniBin** = Unified Binary Architecture

**One binary, multiple modes**:
- CLI commands: `nestgate run|up|down|list|...`
- Server mode: `nestgate daemon` or `nestgate server`
- Auto-daemon: `nestgate-server` (backward compat)

**Key Principle**: One binary can be deployed anywhere (x86_64, ARM, etc.) without separate CLI/server packages

---

## 🏆 **Why UniBin Matters**

### **Pure Rust Enables This!**

**Before Pure Rust**:
- Native dependencies (C libraries)
- Platform-specific builds required
- Separate binaries for each platform
- Cross-compilation difficult

**With Pure Rust** (100% Rust, no C):
- ✅ One binary works everywhere
- ✅ ARM cross-compilation trivial
- ✅ No platform-specific dependencies
- ✅ Single artifact to distribute

---

### **Deployment Benefits**

**Simplicity**:
- ✅ Single binary to distribute
- ✅ One download, one install
- ✅ Simpler version management
- ✅ Reduced binary proliferation

**Operational**:
- ✅ CLI and server always version-matched
- ✅ No version mismatch risks
- ✅ Simpler systemd/service setup
- ✅ Easier rollback (one binary)

**Development**:
- ✅ Shared code between CLI and server
- ✅ Easier consistency
- ✅ Single entry point
- ✅ Simpler CI/CD

---

## 📊 **ToadStool's UniBin Architecture**

### **Status**: First uniBin Primal! (60% complete)

**Implementation** (from pulled updates):

```toml
# crates/cli/Cargo.toml
[[bin]]
name = "toadstool"              # PRIMARY - Modern UniBin
path = "src/main.rs"

[[bin]]
name = "toadstool-cli"          # COMPAT - Legacy alias
path = "src/main.rs"

[[bin]]
name = "toadstool-server"       # COMPAT - Auto-daemon mode
path = "src/main.rs"
```

**Binary Name Detection**:
```rust
// Detect how we were invoked
let bin_name = std::env::args().next()
    .and_then(|p| Path::new(&p).file_name())
    .and_then(|n| n.to_str())
    .unwrap_or("toadstool");

// Auto-daemon for backward compatibility
if bin_name == "toadstool-server" {
    info!("🍄 ToadStool invoked as 'toadstool-server' (legacy mode)");
    return run_server_daemon().await;
}

// Modern command-based routing
let cli = Cli::parse();
match cli.command {
    Commands::Daemon { ... } => run_server_daemon().await,
    Commands::Run { ... } => run_cli_command(...).await,
    // ... other commands
}
```

**Result**: 
- ✅ One source file
- ✅ Three binary names (all from one build)
- ✅ Backward compatibility maintained
- ✅ Modern CLI/daemon routing

---

## 🏗️ **NestGate Current Architecture**

### **Current State**: Single Binary (Server Only)

```
nestgate → Server/daemon binary only
         → No CLI currently
         → Controlled via RPC (tarpc/JSON-RPC)
```

**Cargo.toml**:
```toml
[[bin]]
name = "nestgate"
path = "src/main.rs"
```

**Assessment**:
- ✅ Already single binary!
- ✅ No CLI/server split to merge
- ⚠️ No CLI commands yet
- ✅ Good foundation for uniBin

---

## 🎯 **NestGate UniBin Evolution**

### **Target Architecture**

```
nestgate → CLI commands + daemon mode
```

**Usage**:
```bash
# Daemon mode (current behavior)
nestgate                         # Default: Run daemon
nestgate daemon                  # Explicit daemon mode
nestgate server                  # Alias for daemon

# CLI commands (new!)
nestgate status                  # Check daemon status
nestgate pools list              # List storage pools
nestgate datasets create tank/data  # Create dataset
nestgate snapshots list tank     # List snapshots
nestgate health                  # Health check
nestgate metrics                 # Show metrics
nestgate discover                # Show discovered primals

# Backward compatibility
nestgate-server                  # Auto-daemon (symlink)
```

---

## 📋 **Implementation Plan**

### **Phase 1: CLI Foundation** (4-6 hours)

**Goal**: Add basic CLI commands while maintaining daemon mode

**Tasks**:
1. ✅ Create CLI command structure
   - Add `clap` for argument parsing
   - Define `Commands` enum
   - Add daemon subcommand

2. ✅ Add status/health commands
   - `nestgate status` - daemon status
   - `nestgate health` - health check
   - `nestgate version` - version info

3. ✅ Add storage commands
   - `nestgate pools list`
   - `nestgate pools show <name>`
   - `nestgate datasets list`
   - `nestgate snapshots list`

4. ✅ Update main.rs
   - Add binary name detection
   - Route to CLI or daemon
   - Preserve backward compat

**Code Structure**:
```rust
// src/cli/mod.rs (new)
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as daemon (server mode)
    Daemon {
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Show daemon status
    Status,
    
    /// Health check
    Health,
    
    /// Storage pool commands
    #[command(subcommand)]
    Pools(PoolCommands),
    
    /// Dataset commands
    #[command(subcommand)]
    Datasets(DatasetCommands),
    
    // ... more commands
}

// src/main.rs
#[tokio::main]
async fn main() -> Result<()> {
    // Detect binary name for backward compatibility
    let bin_name = std::env::args().next()
        .and_then(|p| Path::new(&p).file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("nestgate");

    // Auto-daemon if invoked as "nestgate-server"
    if bin_name == "nestgate-server" {
        info!("🏰 NestGate invoked as 'nestgate-server' (legacy mode)");
        return run_daemon_mode().await;
    }

    // Parse CLI arguments
    let cli = Cli::parse();

    match cli.command {
        None => {
            // No subcommand: default to daemon mode
            run_daemon_mode().await
        }
        Some(Commands::Daemon { config }) => {
            run_daemon_mode_with_config(config).await
        }
        Some(command) => {
            execute_cli_command(command).await
        }
    }
}
```

**Effort**: 4-6 hours

---

### **Phase 2: Multi-Binary Config** (1-2 hours)

**Goal**: Build multiple binary names from one source

**Cargo.toml**:
```toml
[[bin]]
name = "nestgate"               # PRIMARY - Modern UniBin
path = "src/main.rs"

[[bin]]
name = "nestgate-server"        # COMPAT - Auto-daemon mode
path = "src/main.rs"
```

**Result**:
- `target/release/nestgate` - Primary binary
- `target/release/nestgate-server` - Auto-daemon symlink

**Effort**: 1-2 hours (mostly testing)

---

### **Phase 3: Advanced CLI Commands** (6-8 hours)

**Goal**: Complete CLI functionality

**Commands to Add**:

**Discovery**:
```bash
nestgate discover primals           # List discovered primals
nestgate discover services          # List discovered services
nestgate discover capabilities      # List capabilities
```

**Metrics**:
```bash
nestgate metrics                    # Show current metrics
nestgate metrics storage            # Storage metrics
nestgate metrics performance        # Performance metrics
```

**Configuration**:
```bash
nestgate config show                # Show current config
nestgate config validate            # Validate config file
nestgate config generate            # Generate default config
```

**Effort**: 6-8 hours

---

### **Phase 4: Cross-Platform Testing** (2-4 hours)

**Goal**: Verify uniBin works across architectures

**Tests**:
1. ✅ Build for x86_64-unknown-linux-gnu
2. ✅ Build for aarch64-unknown-linux-gnu (ARM)
3. ✅ Build for x86_64-apple-darwin (macOS)
4. ✅ Build for aarch64-apple-darwin (M1/M2)
5. ✅ Test binary invocation methods
6. ✅ Test backward compatibility

**Commands**:
```bash
# Build for multiple targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Test binary names
./target/release/nestgate --version
./target/release/nestgate daemon --help
./target/release/nestgate-server --help

# Test CLI commands
./target/release/nestgate status
./target/release/nestgate health
```

**Effort**: 2-4 hours

---

## 💡 **ToadStool Learnings**

### **What We Learned from ToadStool**

**1. Pure Rust is Essential**:
- ✅ NestGate: 100% Pure Rust core (complete!)
- ✅ ToadStool: 100% Pure Rust (complete!)
- ✅ Enables trivial ARM cross-compilation

**2. Incremental Evolution Works**:
- ✅ Phase 1: Architecture (done!)
- ⏳ Phase 2: HTTP cleanup (60% done)
- 📅 Phase 3: Integration (next)
- 📅 Phase 4: Polish (future)

**3. Backward Compatibility Critical**:
- ✅ Binary name detection
- ✅ Auto-daemon mode
- ✅ Legacy aliases
- ✅ Gradual migration path

**4. Documentation is Key**:
- ✅ Clear status documents
- ✅ Phase breakdowns
- ✅ Next steps defined
- ✅ Progress tracking

---

## 📊 **Comparison with ToadStool**

| Aspect | ToadStool | NestGate |
|--------|-----------|----------|
| **Current** | 2 binaries (cli + server) | 1 binary (server only) |
| **Target** | 1 uniBin (multi-mode) | 1 uniBin (multi-mode) |
| **Pure Rust** | ✅ 100% (achieved!) | ✅ 100% (achieved!) |
| **HTTP Removed** | ⏳ 60% | ✅ 100% |
| **UniBin Progress** | ⏳ 60% (in progress) | 📅 Not started |
| **CLI Commands** | ✅ Extensive | ❌ None yet |
| **Daemon Mode** | ✅ Working | ✅ Working |
| **ARM Ready** | ✅ Yes | ✅ Yes |
| **Advantage** | Head start on CLI | Simpler (no merge needed!) |

**Key Insight**: NestGate has an EASIER path than ToadStool!
- No binary merge required (already single binary)
- Just add CLI commands to existing daemon
- Pure Rust already 100% (vs ToadStool's 60% HTTP cleanup)

---

## 🎯 **Why NestGate Should Adopt UniBin**

### **Strategic Benefits**

**1. Ecosystem Alignment**:
- ✅ Follow ToadStool's leadership
- ✅ Match production standard
- ✅ Modern architecture pattern

**2. Pure Rust Advantage**:
- ✅ We have 100% pure Rust (ToadStool at 60%)
- ✅ Can demonstrate ARM deployment first!
- ✅ Showcase pure Rust benefits

**3. User Experience**:
- ✅ Simpler deployment
- ✅ Better CLI tools
- ✅ Self-documenting interface

**4. Operational Benefits**:
- ✅ Single artifact to manage
- ✅ Version consistency guaranteed
- ✅ Easier troubleshooting

---

## 📅 **Timeline**

### **Total Effort**: 13-20 hours

**Phase 1** (CLI Foundation): 4-6 hours
**Phase 2** (Multi-Binary): 1-2 hours
**Phase 3** (Advanced CLI): 6-8 hours
**Phase 4** (Cross-Platform): 2-4 hours

**Suggested Schedule**:
- Session 1 (4-6 hours): Phase 1 complete
- Session 2 (4-6 hours): Phase 2 + part of Phase 3
- Session 3 (4-6 hours): Complete Phase 3 + Phase 4

---

## 🚀 **Quick Start Option**

### **Minimal UniBin** (2-3 hours)

**Goal**: Basic uniBin with essential commands only

**Commands**:
- `nestgate` (default: daemon)
- `nestgate daemon` (explicit daemon)
- `nestgate status` (daemon status)
- `nestgate health` (health check)
- `nestgate version` (version info)

**Benefit**: Get uniBin working quickly, add more commands later

**Approach**: Implement Phase 1 only (minimal commands)

---

## 💪 **Competitive Advantage**

### **NestGate Can Lead!**

**Why NestGate is Best Positioned**:

1. ✅ **100% Pure Rust** (ToadStool at 60%)
   - We completed HTTP removal TODAY!
   - ToadStool still has 40% HTTP cleanup left

2. ✅ **Simpler Evolution Path**
   - Already single binary (no merge needed)
   - Just add CLI commands (easier)
   - ToadStool: Merge two binaries (harder)

3. ✅ **Proven Architecture**
   - DashMap migration (43/406, 10.6%)
   - 10-25x performance gains
   - Modern async patterns

4. ✅ **ARM Ready**
   - Zero C dependencies
   - Trivial cross-compilation
   - Can demonstrate first!

**Opportunity**: Complete uniBin BEFORE ToadStool finishes HTTP cleanup!

---

## 🎊 **Recommendation**

### **Path Forward**: **ADOPT UNIBIN** ✅

**Priority**: Medium-High

**Why**:
1. ✅ ToadStool sets the standard (first uniBin primal)
2. ✅ NestGate can follow and excel (100% pure Rust advantage)
3. ✅ Modern architecture aligns with ecosystem
4. ✅ ARM cross-compilation showcase
5. ✅ Better user experience

**Timing**: After DashMap migration or in parallel

**Effort**: 13-20 hours (manageable across 3 sessions)

**Benefit**: Production-standard architecture, ecosystem leadership

---

## 📚 **Next Steps**

### **Immediate**

1. ✅ Review this plan
2. ✅ Decide on timeline
3. ✅ Prioritize vs DashMap migration

### **Phase 1** (When Ready)

1. Add `clap` dependency
2. Create `src/cli/` module
3. Implement basic commands
4. Update main.rs
5. Test and document

### **Long-Term**

1. Complete all CLI commands
2. Cross-platform testing
3. ARM deployment demo
4. Ecosystem showcase

---

## 🏆 **Success Criteria**

**UniBin Complete When**:
- [ ] Single binary with CLI + daemon
- [ ] Multiple binary names working
- [ ] Backward compatibility verified
- [ ] Essential CLI commands implemented
- [ ] ARM cross-compilation tested
- [ ] Documentation updated
- [ ] biomeOS integration verified

---

## 📖 **References**

**ToadStool Documents**:
- `UNIBIN_STATUS_JAN_16_2026.md` - Status and design
- `UNIBIN_PHASE1_STATUS_JAN_16_2026.md` - Implementation details
- `PURE_RUST_ARCHITECTURE_ACHIEVED_JAN_16_2026.md` - Pure Rust benefits

**ToadStool Commits**: 249 new commits pulled (Jan 16, 2026)

**Key Insights**:
- UniBin is production standard
- Pure Rust enables cross-platform uniBin
- Backward compatibility is essential
- Incremental evolution works

---

## 🎯 **Conclusion**

**UniBin Status**: ✅ **Adopt Recommended**

**Rationale**:
- ToadStool leads with uniBin (ecosystem standard)
- NestGate has advantages (100% pure Rust, simpler path)
- Production-ready architecture pattern
- Better user experience and deployment

**Timeline**: 13-20 hours across 3 sessions

**Priority**: Medium-High (after or parallel to DashMap)

**Result**: NestGate becomes second uniBin primal, showcases pure Rust ARM deployment!

---

**Created**: January 16, 2026, 3:30 AM  
**Source**: ToadStool uniBin evolution (249 commits pulled)  
**Purpose**: Plan NestGate adoption of unified binary standard  
**Status**: Ready for review and execution! ✅

🦀 **PURE RUST ENABLES UNIBIN** | 🏆 **FOLLOW TOADSTOOL'S LEAD** | ⚡ **ARM READY**

---

**Next**: Review plan, decide timing, execute when ready! 🚀✨
