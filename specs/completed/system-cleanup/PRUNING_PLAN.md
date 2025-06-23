# Root Directory Pruning Plan

## Overview

This document outlines the plan to clean up the numerous files still present at the root of the NestGate repository. Each file will be categorized and either moved to an appropriate location within our new structure or deleted if redundant.

## File Categorization and Actions

### Implementation Files to Move to crates/nestgate-ui/

- `server.ts`, `server.js` → crates/nestgate-ui/server/
- `api-server.ts`, `api-server.js` → crates/nestgate-ui/server/
- `vite.config.ts` → crates/nestgate-ui/config/
- `tsconfig.json`, `tsconfig.server.json`, `tsconfig.node.json` → crates/nestgate-ui/config/
- `babel.config.js`, `jest.config.js` → crates/nestgate-ui/config/
- `fix-package-json.js` → crates/nestgate-ui/scripts/
- `build.rs` → crates/nestgate-core/ (if it's a Rust build file)

### Documentation Files to Move to docs/ or specs/

#### Project Management (specs/project/)
- `NEXT-STEPS.md` → specs/project/next_steps.md
- `IMPLEMENTATION-PROGRESS.md` → specs/project/implementation_progress.md
- `UI_HALF_MARATHON_PROGRESS.md` → specs/project/ui_marathon_progress.md
- `README_OCTOBER_SPRINT.md` → specs/project/october_sprint.md
- `SEGREGATION-PLAN.md` → specs/project/segregation_plan.md

#### Architecture (specs/architecture/)
- `README-NEW-ARCHITECTURE.md` → specs/architecture/new_architecture.md
- `CODEBASE.md` → specs/architecture/codebase_overview.md

#### Implementation (specs/implementation/)
- `live-mode-cleanup-summary.md` → specs/implementation/live_mode_cleanup.md
- `cleanup-summary.md` → specs/implementation/cleanup_summary.md
- `live-only-implementation-guide.md` → specs/implementation/live_only_guide.md
- `live-mode-implementation-plan.md` → specs/implementation/live_mode_plan.md
- `live-mode-summary.md` → specs/implementation/live_mode_summary.md
- `port-manager-implementation-summary.md` → specs/implementation/port_manager_summary.md
- `PORT_MANAGER_INTEGRATION_SUMMARY.md` → specs/implementation/port_manager_integration.md
- `PORT_MANAGER_README.md` → specs/implementation/port_manager_readme.md
- `STARTUP.md` → specs/implementation/startup_guide.md
- `ANGULAR_REMOVAL_GUIDE.md` → specs/implementation/angular_removal.md

#### Development Guides (docs/guides/)
- `DEPENDENCIES.md` → docs/guides/dependencies.md
- `AGENTS.md` → docs/guides/agents.md
- `TEAMCHAT.md` → docs/guides/teamchat.md (if user-facing, otherwise delete)

### Scripts to Move to scripts/ Directory

#### Build Scripts (scripts/build/)
- `build.sh` → scripts/build/

#### Start Scripts (scripts/start/)
- `start.sh`, `start-dev.sh`, `start-live-mode.sh`, `start-live-server.sh`, `start-strict-mode.sh` → scripts/start/
- `dev.sh` → scripts/start/

#### Test Scripts (scripts/test/)
- `run-tests.sh`, `e2e-test.sh`, `test_crates.sh`, `test_lib_crates.sh` → scripts/test/
- `check-server.sh` → scripts/test/

#### Migration Scripts (scripts/migration/)
- Create a new directory: scripts/migration/
- `finalize_migration.sh`, `cleanup_src.sh`, `update_imports.sh`, `migrate_remaining.sh`, `analyze_src.sh`, `fix_remaining_issues.sh`, `validate_imports.sh`, `migrate.sh` → scripts/migration/

#### Other Scripts (scripts/util/)
- `generate_ai_training_workload.sh` → scripts/util/

### Configuration Files

- `.npmrc` → Keep at root (npm configuration)
- `package.json`, `package-lock.json` → Keep at root (npm package configuration)
- `Cargo.toml`, `Cargo.lock` → Keep at root (Rust package configuration)
- `rustfmt.toml`, `.clippy.toml` → Keep at root (Rust tooling configuration)
- `.gitignore` → Keep at root (git configuration)
- `Dockerfile` → Move to .dev-tools/docker/
- `Cargo.toml.bak` → Delete (backup file)

### Directories to Review

- `src/` → Move remaining files to crates/nestgate-ui/src/
- `server/` → Move to crates/nestgate-ui/server/
- `migration_plan/` → Move to specs/implementation/migration/
- `middleware/` → Move to crates/nestgate-middleware/ if it's implementation code
- `config/` → Review contents and move appropriately
- `docker/` → Move to .dev-tools/docker/ (contains Docker-related files for development and testing)
- `tempRules/`, `templates/` → Keep as is (workflow-related)

### Log/Generated Files to Delete

- `run.log`, `ai_tuning.log` → Delete (log files)
- `.service-info`, `.port-manager-pid` → Temporary files, can be deleted

## Implementation Steps

1. Create any missing target directories
2. Move files to their designated locations
3. Update any relative imports in moved files
4. Delete redundant or temporary files
5. Update README.md references if needed
6. Verify the system still builds and runs
7. Commit changes with clear message about pruning

## Success Criteria

- Root directory contains only essential configuration files and directories
- All implementation code is in appropriate crates/ subdirectories
- All documentation is in either docs/ or specs/
- All scripts are in scripts/ subdirectories
- System builds and runs successfully 