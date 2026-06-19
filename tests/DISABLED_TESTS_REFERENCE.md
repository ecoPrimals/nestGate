# 🚫 Disabled Tests Reference

**Last Updated**: Jun 19, 2026  
**Status**: Active

---

## `.disabled` test files

There are **no** `*.disabled` test files in the repository. Nothing is excluded from compilation by renaming sources to `.disabled`.

To confirm:

```bash
find . -name "*.disabled" -type f
# (expect no test sources; only unrelated tooling if any)
```

---

## Ignored tests (`#[ignore]`)

Tests marked `#[ignore]` are compiled and shipped with the suite but **not** run by default. Most need real infrastructure, privileged operations, or long-running setup (chaos suites, live storage/network, benchmarks).

Run them explicitly when your environment matches the test’s assumptions:

```bash
cargo test --workspace --all-features -- --ignored
```

Narrow by name or test binary as usual, e.g. filters on scenario or module path.

---

## Where to run focused suites

Use the layout under `tests/`:

- **`tests/chaos/`** — chaos harnesses and scenarios (`comprehensive_chaos_tests.rs`, `advanced_resilience_tests.rs`, `network_failure_scenarios.rs`, …).
- **`tests/integration_test_suite/`** — comprehensive integration modules and helpers (see also `integration_test_suite_main.rs`).
- **Top-level `tests/*.rs`** — Cargo integration test crates (e2e scenarios, fault-injection suites, performance batteries, …).

See [README.md](README.md) for organization and **`cargo test --workspace`** (full suite including root binaries) versus **`cargo test --workspace --lib`** (crate library tests only).

---

## Current metrics (reference)

| Metric | Value |
|--------|--------|
| Passing | 12,941 |
| Failed | 0 |
| Ignored | 427 |
| Workspace coverage (line) | 84%+ |

Re-run `cargo test --workspace` and `cargo llvm-cov` after large changes; numbers drift with the tree.

---

## Maintenance

- Prefer `#[ignore]` with a short comment over ad hoc `.disabled` filenames unless a file truly cannot compile.
- When adding long-running or environment-specific tests, mark them ignored and document required setup beside the test or in the relevant `tests/<category>/` README.
- If a source is temporarily renamed to `.disabled`, restore it or delete it, and document the reason in this file.

---

**Maintained By**: Development Team  
**Next Review**: When ignored-test count or infra assumptions change materially
