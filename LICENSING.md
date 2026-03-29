# Licensing alignment (scyBorg)

This file summarizes how **NestGate** sits in the ecoPrimals **scyBorg** provenance model. It does not replace the legal text in [`LICENSE`](LICENSE).

## Provenance trio (scyBorg)

scyBorg is the ecosystem triple-copyleft stack:

| Layer | License | Covers |
|-------|---------|--------|
| **Software** | AGPL-3.0 | Code, shaders, tools, infrastructure |
| **Game mechanics** | [ORC](https://azoralaw.com/orclicense/) Licensed Material | Rules, stats, progression, encounter math |
| **Creative** | [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) | Art, narrative, docs, worlds, music, maps |

ORC also defines **Reserved Material** (e.g. studio branding, trademarks); that layer does not replace code or mechanics licensing.

Machine-verifiable attribution and derivation are intended to be enforced via the **provenance trio** (sweetGrass, rhizoCrypt, loamSpine) as described in ecoPrimals scyBorg guidance.

## This repository: code layer

NestGate follows the [GNU Affero General Public License v3.0 only](https://www.gnu.org/licenses/agpl-3.0.html) (`AGPL-3.0-only`) — see `LICENSE`. That choice matches ecoPrimals **scyBorg** guidance for this codebase: the SPDX identifier for NestGate is **AGPL-3.0-only**, not AGPL-3.0-or-later.

The broader wateringHole documentation may still reference **AGPL-3.0-or-later** in places; the ecosystem is **aligning on AGPL-3.0-only** for primals repositories, and NestGate is already on that line.

## Beardog entropy provision

The `LICENSE` file includes a **beardog entropy** special provision: humans accessing this software through beardog entropy systems receive free use for personal, educational, and non-commercial purposes, alongside the stated AGPL-3.0-only and network-use obligations.

## Where to read the full rules

- **Legal terms for this repo:** [`LICENSE`](LICENSE)
- **Ecosystem standard and layers:** `infra/wateringHole/SCYBORG_PROVENANCE_TRIO_GUIDANCE.md` (ecoPrimals infra)
