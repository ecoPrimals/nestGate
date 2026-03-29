# Licensing (scyBorg Provenance Trio)

This file describes how **NestGate** materializes the ecoPrimals **scyBorg** provenance model per the wateringHole standard:

`infra/wateringHole/SCYBORG_PROVENANCE_TRIO_GUIDANCE.md` (ecoPrimals infra).

It summarizes scope; the legal texts are in the license files named below.

## scyBorg triple (ecosystem standard)

scyBorg is the triple-copyleft stack for ecoPrimals:

| Layer | License | Typical coverage |
|-------|---------|------------------|
| **Software** | AGPL-3.0 | Code, shaders, build config, infrastructure |
| **Game mechanics** | [ORC](https://azoralaw.com/orclicense/) Licensed Material | Rules, stats, progression, encounter math |
| **Creative** | [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) | Art, narrative, documentation, specs, diagrams |

ORC also defines **Reserved Material** (e.g. branding, trademarks); that layer does not replace code or mechanics licensing.

Machine-verifiable attribution and derivation are intended to be enforced via the **provenance trio** (sweetGrass, rhizoCrypt, loamSpine) as described in the scyBorg guidance.

## This repository: NestGate (infrastructure primal)

NestGate is an **infrastructure** primal, not a game product. This repository does not ship standalone game rules, stat blocks, progression systems, or encounter math as **ORC Licensed Material**. The ORC layer of the full triple therefore **does not apply** here in practice.

For NestGate, the scyBorg materialization **simplifies** to:

| Effective layer | License | Legal text in repo |
|-----------------|---------|---------------------|
| **Software** | AGPL-3.0-only | [`LICENSE`](LICENSE) |
| **Documentation and creative content** | CC-BY-SA 4.0 | [`LICENSE-CC-BY-SA-4.0`](LICENSE-CC-BY-SA-4.0) |

SPDX for the Rust crates remains **AGPL-3.0-only** (see `Cargo.toml`); that identifier applies to **code**, not to non-code docs.

## Which files use which license

Use these patterns unless a file explicitly states otherwise:

### AGPL-3.0-only (see [`LICENSE`](LICENSE))

- Rust and other **program source** under `code/`, `src/`, `tests/**/*.rs`, `benches/`, and `tools/**` (source and build scripts)
- **Build and package metadata**: `Cargo.toml`, `Cargo.lock`, workspace manifests, typical CI/build configs that are code-like (e.g. `.github` workflows as source)
- **Generated or binary artifacts** are governed by the license of the sources that produce them; the published **software** remains AGPL-3.0-only as stated in `LICENSE`

### CC-BY-SA 4.0 (see [`LICENSE-CC-BY-SA-4.0`](LICENSE-CC-BY-SA-4.0))

- **Markdown and prose documentation** outside of executable source: e.g. root `*.md` (including this file), `docs/**`, `specs/**`, `examples/**/*.md`, `tests/**/*.md`, `benches/README.md`, and `README.md` / guide files under `code/crates/**` and `tools/**` when they are documentation, not code
- **Specifications and integration notes** expressed as prose or diagrams in those trees

If a path is ambiguous (e.g. a `.md` file that is mostly embedded license text or a tiny pointer), prefer the intent: **prose/spec/docs → CC-BY-SA 4.0; executable source and build config → AGPL-3.0-only**.

### ORC

- **Not used** for NestGate content as described above. Repositories that add game-mechanical material under ecoPrimals should apply ORC per `SCYBORG_PROVENANCE_TRIO_GUIDANCE.md`.

## Beardog entropy provision

The [`LICENSE`](LICENSE) file includes a **beardog entropy** special provision: humans accessing this software through beardog entropy systems receive free use for personal, educational, and non-commercial purposes, alongside the stated AGPL-3.0-only and network-use obligations. That provision applies to the **software** layer, not to a separate re-licensing of CC-BY-SA documentation.

## SPDX and alignment note

NestGate uses **AGPL-3.0-only** (not AGPL-3.0-or-later) for its crates, consistent with this repository’s `LICENSE` and `Cargo.toml`. Broader ecosystem docs may still mention **AGPL-3.0-or-later** in places; this repo’s SPDX line is **AGPL-3.0-only** for **code**.

## Where to read the full rules

| Topic | Location |
|-------|----------|
| Software (AGPL-3.0-only) | [`LICENSE`](LICENSE) |
| Documentation / creative (CC-BY-SA 4.0) | [`LICENSE-CC-BY-SA-4.0`](LICENSE-CC-BY-SA-4.0) |
| Ecosystem scyBorg standard | `infra/wateringHole/SCYBORG_PROVENANCE_TRIO_GUIDANCE.md` |
