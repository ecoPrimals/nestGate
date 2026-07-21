# Session 125: Procfs Consolidation Phase 3 + Dep Bumps

**Date**: Jul 21, 2026  
**Wave**: 150t  
**Commit**: `b98aca49`

## Changes

### Procfs Consolidation Phase 3

Eliminated 17 scattered `/proc` read callsites in `nestgate-api` by
delegating to `nestgate_platform::linux_proc`:

- `handlers/hardware_tuning/linux_proc.rs` (428→235L): CPU, memory,
  network, disk metrics now delegate to centralized `linux_proc`; GPU
  detection and sysfs profiles retained as domain-specific.
- `handlers/metrics_collector/linux_proc.rs` (352→170L): CPU, memory,
  network reads replaced; ZFS ARC + pool metrics retained.
- `handlers/hardware_tuning/types/monitors.rs`: `CpuMonitor`,
  `MemoryMonitor`, `NetworkMonitor` implementations → `linux_proc`.
- `handlers/hardware_tuning/handlers.rs`: `detect_cpu_info` and
  `detect_memory_info` → `linux_proc`.
- `handlers/performance_dashboard/metrics/metrics_system.rs` (186→97L):
  All system metrics via `linux_proc`; disk I/O retained (async sector
  parsing).

Production `/proc` reads: 58→41 (remaining: 8 canonical linux_proc,
8 ZFS arcstats, and domain-specific niche reads).

### Dependency Bumps

6 patch-level updates: tokio 1.53.0→1.53.1, libc 0.2.186→0.2.188,
tokio-util 0.7.18→0.7.19, zerocopy 0.8.54→0.8.55.

### Clippy Cleanup

- `mem_total_gib` unnecessary `Result` wrapping removed (now returns
  `u32` directly)
- Unfulfilled `clippy::similar_names` expect removed from `lib.rs`

## Scorecard

- Tests: 1,630 passed, 80 ignored
- Clippy: 0 warnings
- Net delta: -493 lines
