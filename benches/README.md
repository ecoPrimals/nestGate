# NestGate Benchmarks — Root (Legacy)

The workspace root `Cargo.toml` does not define any `[[bench]]` entries for the `.rs`
files in this directory, so they are **not built** when you run `cargo bench` from the
repository root.

Active Criterion benchmarks live under crate packages — for example
`code/crates/nestgate-core/Cargo.toml` registers `performance_validation`,
`comprehensive_reality_check`, and `working_performance_benchmark`.

These files are retained as reference sources for the DashMap migration era (Jan 2026).
The migration is complete; DashMap is in use across the codebase.
