# UPSTREAM BUG REPORT: Inverted Boolean in Socket-Only Mode

**Date**: February 9, 2026
**Reporter**: biomeOS Core Team (Tower Atomic validation)
**Severity**: CRITICAL (blocks socket-only daemon startup)
**Status**: PATCHED DOWNSTREAM (temporary fix in biomeOS fork)
**Patch Location**: `code/crates/nestgate-bin/src/cli.rs` line ~337

---

## The Bug

The Feb 9 evolution introduced a `socket_only` field alongside the existing `enable_http` field in the `Daemon` command. The `use_socket_only` boolean is then passed directly to `run_daemon()` whose 4th parameter is `enable_http: bool`. Since both are `bool`, Rust doesn't catch the semantic inversion.

### Pre-Evolution Code (CORRECT)

Before the Feb 9 session, the CLI had only `enable_http`:

```rust
Daemon {
    #[arg(long)]
    enable_http: bool,  // defaults to false
}

// Dispatched correctly:
Commands::Daemon { port, bind, dev, enable_http } => {
    crate::commands::service::run_daemon(port, &bind, dev, enable_http)
}
```

`nestgate daemon` -> `enable_http = false` -> socket-only mode. Correct.

### Post-Evolution Code (BUG INTRODUCED)

The Feb 9 evolution added `socket_only` with `default_value_t = true`:

```rust
Daemon {
    #[arg(long, default_value_t = true)]
    socket_only: bool,           // defaults to true

    #[arg(long, conflicts_with = "socket_only")]
    enable_http: bool,           // defaults to false
}

// Bug is HERE:
Commands::Daemon { port, bind, dev, socket_only, enable_http } => {
    let use_socket_only = socket_only && !enable_http;  // = true && !false = true

    // use_socket_only (true) is passed as enable_http parameter!
    crate::commands::service::run_daemon(port, &bind, dev, use_socket_only)
    //                                                      ^^^^^^^^^^^^^^
    //                                                      enable_http receives `true`
    //                                                      -> starts HTTP mode!
}
```

### What happens

```
$ nestgate daemon --socket-only

INFO 🔌 Starting NestGate in Unix socket-only mode    <-- log says socket-only
INFO 🌐 Starting NestGate with HTTP server             <-- but starts HTTP mode!
INFO    Port: 8080, Bind: 0.0.0.0

🚀 Starting NestGate HTTP service on 127.0.0.1:8080   <-- HTTP, not socket
```

No `nestgate.sock` is created. The `storage.*` JSON-RPC methods are only available via the Unix socket server, so they become unreachable.

### Why the investigation said "FALSE POSITIVE"

The investigation document (`MODEL_CACHE_BUG_INVESTIGATION_FEB_9_2026.md`) analyzed the **pre-evolution** code that only had `enable_http`. That code was indeed correct. But the evolution session then added the `socket_only` field and introduced this inversion -- the investigation was looking at stale code.

---

## Reproduction

```bash
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)

# Expected: socket-only mode, creates nestgate.sock
nestgate daemon --socket-only

# Actual: starts HTTP server on port 8080, no socket created
# Test:
ls /run/user/1000/biomeos/nestgate.sock  # -> NOT FOUND
curl http://127.0.0.1:8080/health        # -> responds (should NOT exist)
```

---

## Second Occurrence: `main.rs` Legacy Symlink Path

The same inversion exists in `code/crates/nestgate-bin/src/main.rs` line ~53, the `nestgate-server` legacy symlink handler:

```rust
// BUG (main.rs):
return nestgate_bin::commands::service::run_daemon(
    nestgate_core::defaults::network::DEFAULT_API_PORT,
    nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS,
    false,
    true, // LABELED "socket_only = true" but parameter IS "enable_http"
)

// PATCHED:
    false, // enable_http = false -> socket-only mode (correct default)
```

---

## Downstream Patches (Temporary)

biomeOS has applied fixes to both locations:

**cli.rs line ~337** (CLI `nestgate daemon --socket-only`):
```rust
// BEFORE (bug):
crate::commands::service::run_daemon(port, &bind, dev, use_socket_only)

// AFTER (patched):
crate::commands::service::run_daemon(port, &bind, dev, !use_socket_only)
```

**main.rs line ~53** (legacy `nestgate-server` symlink):
```rust
// BEFORE (bug):
    true, // socket_only = true (default per PRIMAL_DEPLOYMENT_STANDARD)

// AFTER (patched):
    false, // enable_http = false -> socket-only mode (default)
```

---

## Recommended Proper Fix

The cleanest fix eliminates the semantic confusion by removing the dual-boolean pattern entirely. Two options:

### Option A: Remove `socket_only`, keep only `enable_http` (SIMPLEST)

```rust
Daemon {
    #[arg(long)]
    enable_http: bool,  // false = socket-only (default), true = HTTP mode
}

Commands::Daemon { port, bind, dev, enable_http } => {
    crate::commands::service::run_daemon(port, &bind, dev, enable_http)
}
```

This was the pre-evolution design and it was correct. The `socket_only` field is redundant since socket-only is already the default when `enable_http` is false.

### Option B: Use an enum instead of booleans

```rust
#[derive(Clone, Debug, ValueEnum)]
enum DaemonMode {
    Socket,
    Http,
}

Daemon {
    #[arg(long, default_value_t = DaemonMode::Socket)]
    mode: DaemonMode,
}
```

This makes the semantics explicit and eliminates boolean confusion entirely.

### Option C: Rename `run_daemon` parameter (MINIMAL)

```rust
pub async fn run_daemon(port: u16, bind: &str, dev: bool, socket_only: bool)
//                                                         ^^^^^^^^^^^^^
// Now the parameter name matches the caller's intent
```

Then `use_socket_only` passes through correctly without negation.

---

## Validated Behavior After Patch

```bash
$ nestgate daemon --socket-only

INFO 🔌 Starting NestGate in Unix socket-only mode
INFO 🔌 Starting NestGate in socket-only mode (TRUE ecoBin - default)

Socket: /run/user/1000/biomeos/nestgate.sock    # CREATED
Protocol: JSON-RPC 2.0 over Unix socket

# storage.exists WORKS:
echo '{"jsonrpc":"2.0","method":"storage.exists","params":{"family_id":"8ff3b864a4bc589a","key":"test"},"id":1}' \
  | nc -U /run/user/1000/biomeos/nestgate.sock
# -> {"jsonrpc":"2.0","result":{"exists":false},"id":1}

# storage.store WORKS:
# -> {"jsonrpc":"2.0","result":{"success":true,"size_bytes":33},"id":2}

# storage.retrieve WORKS:
# -> {"jsonrpc":"2.0","result":{"data":{"model":"TinyLlama","size":2100}},"id":3}
```

All three storage methods confirmed working in socket-only mode after the patch.

---

## Impact on biomeOS

With this patch applied:

- Nest Atomic (Tower + NestGate) is fully operational
- `storage.exists` (new method from this evolution) works correctly
- `storage.retrieve` returns actual data (Bug 2 was likely also caused by this -- retrieve via HTTP has different codepath than via socket)
- Model cache `find_on_mesh()` now uses `storage.exists` -> `storage.retrieve` two-phase lookup
- All primals synced across Tower, gate2, USB, and Pixel8a deploy directories

**biomeOS will carry the downstream patch until NestGate's next evolution resolves this upstream.**
