# NestGate Benchmarks — Root

Active Criterion benchmarks live under crate packages:

- `code/crates/nestgate-core/benches/` — `performance_validation`, `comprehensive_reality_check`, `working_performance_benchmark`
- `code/crates/nestgate-zfs/benches/` — `performance_benchmarks`

Run with `cargo bench -p nestgate-core` or `cargo bench -p nestgate-zfs`.

Legacy DashMap-migration-era benchmarks (Jan 2026) were removed in Session 81
(May 29, 2026) — they were not registered in root `Cargo.toml` and did not build.
Git history preserves them as fossil record.
