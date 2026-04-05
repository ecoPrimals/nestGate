# 🚫 Disabled Tests Reference

**Last Updated**: April 5, 2026  
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

Tests marked `#[ignore]` are compiled and shipped with the suite but **not** run by default. Most need real infrastructure, privileged operations, or long-running setup (e.g. e2e, chaos, live storage/network).

Run them explicitly when your environment matches the test’s assumptions:

```bash
cargo test --workspace --all-features -- --ignored
```

Narrow by name or test binary as usual, e.g. filters on scenario or module path.

---

## Where to run focused suites

Use the layout and frameworks under `tests/`:

- **`tests/e2e/`** — enhanced e2e framework and workflows (`enhanced_e2e_framework.rs`, `framework/`, `workflows/`, …).
- **`tests/chaos/`** — chaos harnesses and scenarios (`chaos_testing_framework.rs`, `enhanced_chaos_framework.rs`, …).
- **`tests/fault/`** — fault-injection-oriented tests.
- **`tests/performance/`** — performance-oriented modules.

See [README.md](README.md) for the full category list and `cargo test --workspace --all-features` as the main entry point.

---

## Current metrics (reference)

| Metric | Value |
|--------|--------|
| Passing (total) | ~11,821 |
| Failed | 0 |
| Ignored | ~463 |
| Workspace coverage (line) | ~80% |

Re-run `cargo test --workspace` and `cargo llvm-cov` after large changes; numbers drift with the tree.

---

## Maintenance

- Prefer `#[ignore]` with a short comment over ad hoc `.disabled` filenames unless a file truly cannot compile.
- When adding long-running or environment-specific tests, mark them ignored and document required setup beside the test or in the relevant `tests/<category>/` README.
- If a source is temporarily renamed to `.disabled`, restore it or delete it, and document the reason in this file.

---

**Maintained By**: Development Team  
**Next Review**: When ignored-test count or infra assumptions change materially
