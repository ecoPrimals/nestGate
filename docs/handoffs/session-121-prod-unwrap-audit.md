# Session 121: Production Unwrap Deep Audit

**Wave**: 150d | **Date**: Jul 18, 2026 | **Primal**: nestGate

## Summary

Complete production unwrap/expect audit across all 14 nestGate crates. Confirms
the scorecard "Prod unwrap" column with audited numbers.

## Audit Results

**0 `.unwrap()` in production. 10 `.expect()` in production.**

All 10 are justified, annotated with `#[expect(clippy::expect_used, reason = "...")]`,
and documented with `# Panics` sections where applicable.

### nestgate-rpc (5 expect)

All are `OnceLock::get_or_init` lazy initialization:

| File | Function | Pattern |
|------|----------|---------|
| `content_ops.rs:32` | `shared_state()` | `StorageState::new().expect(...)` |
| `coord_ops.rs:25` | `shared_state()` | `StorageState::new().expect(...)` |
| `footprint_ops.rs:25` | `shared_state()` | `StorageState::new().expect(...)` |
| `semantic_router/content.rs:31` | `shared_state()` | `StorageState::new().expect(...)` |
| `unix_adapter_handlers.rs:375` | `content_state()` | `ContentState::new().expect(...)` |

These return `&'static T` — the `OnceLock` init closure cannot propagate errors
with `?` without restructuring to fallible lazy init.

### nestgate-core (4 expect)

All are structural invariant guards on pool handles:

| File | Function | Pattern |
|------|----------|---------|
| `safe_memory_pool.rs:202` | `value()` | `opt.as_mut().expect("PoolHandle invariant")` |
| `safe_memory_pool.rs:218` | `value_mut()` | `opt.as_mut().expect("PoolHandle invariant")` |
| `safe_memory_pool.rs:237` | `into_inner()` | `opt.take().expect("PoolHandle invariant")` |
| `advanced_optimizations.rs:189` | `as_mut_slice()` | `slot_guard.as_mut().expect("PoolBlockGuard invariant")` |

A `PoolHandle` can only exist for occupied slots — the invariant is maintained
by construction. Returning `Result` would burden all callers unnecessarily.

### nestgate-api (1 expect)

| File | Function | Pattern |
|------|----------|---------|
| `rest/handlers/zfs/helpers.rs:78` | `unknown_timestamp()` | `DateTime::from_timestamp(0, 0).expect(...)` |

`const fn` in deprecated REST layer. `1970-01-01T00:00:00Z` is always representable.

### All other crates (0)

nestgate-bin, nestgate-config, nestgate-discovery, nestgate-security,
nestgate-storage, nestgate-zfs, nestgate-types, nestgate-platform,
nestgate-installer, nestgate-performance, nestgate-canonical: **zero**
production `.unwrap()` or `.expect()`.

## Scorecard Update

The nestGate scorecard column values are now confirmed:

| Dimension | Value | Status |
|-----------|-------|--------|
| Tests | 1,710 (1,630 pass + 80 ignored) | Confirmed |
| Clippy | 0 | Confirmed |
| Fmt | 0 | Confirmed |
| Debt | 0 | Confirmed |
| Unsafe | 0 | Confirmed |
| >800L | 0 | Confirmed |
| Prod unwrap | 10 | **AUDITED** — 0 unwrap + 10 justified expect |

## Ecosystem Context (Wave 150d)

- Three-domain model formalized: `nestgate.io` as data service point
- Both live products (footPrint, esotericWebb) have P0 routing issues — cellMembrane/ops owned
- Subdomain standard: `prefix.primals.eco` required for all compositions
- Public Surface + Compositions dimensions RED (routing broken, not nestGate code)
