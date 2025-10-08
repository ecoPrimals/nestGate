#!/bin/bash
# cleanup-archive-code.sh
# Safe cleanup of old backup code while preserving documentation

set -e

echo "🧹 NestGate Archive Code Cleanup"
echo "================================"
echo ""

# Safety check: Are we in the right directory?
if [ ! -f "Cargo.toml" ] || [ ! -d "code/crates/nestgate-core" ]; then
    echo "❌ Error: Must run from nestgate root directory"
    exit 1
fi

echo "📊 Current State:"
if [ -d "backups" ]; then
    BACKUP_SIZE=$(du -sh backups/ 2>/dev/null | cut -f1)
    BACKUP_FILES=$(find backups -name "*.rs" 2>/dev/null | wc -l)
    echo "   backups/: $BACKUP_SIZE ($BACKUP_FILES Rust files)"
else
    echo "   backups/: Not found"
    BACKUP_SIZE="0"
fi

if [ -d "archive" ]; then
    ARCHIVE_SIZE=$(du -sh archive/ 2>/dev/null | cut -f1)
    echo "   archive/: $ARCHIVE_SIZE (will be preserved)"
else
    ARCHIVE_SIZE="0"
fi

ACTIVE_BACKUPS=$(find code/crates -name "*.backup-*" -o -name "*.pedantic_backup" -o -name "*.old" 2>/dev/null | wc -l)
echo "   Active code backup files: $ACTIVE_BACKUPS"
echo ""

# Phase 1: Archive backups directory (optional safety)
if [ -d "backups" ] && [ "$BACKUP_SIZE" != "0" ]; then
    echo "📦 Phase 1: Archive backups/ directory (optional)"
    echo "   Current size: $BACKUP_SIZE"
    echo ""
    read -p "   Create tar.gz archive before deletion? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "   Creating backups-archive-oct5-2025.tar.gz..."
        tar -czf backups-archive-oct5-2025.tar.gz backups/
        ARCHIVE_RESULT_SIZE=$(du -sh backups-archive-oct5-2025.tar.gz | cut -f1)
        echo "   ✅ Archive created: backups-archive-oct5-2025.tar.gz ($ARCHIVE_RESULT_SIZE)"
    else
        echo "   ⏭️  Skipped archive creation"
    fi
    echo ""
fi

# Phase 2: Delete backups directory
if [ -d "backups" ]; then
    echo "🗑️  Phase 2: Delete backups/ directory"
    echo "   ⚠️  This will delete $BACKUP_SIZE of old code backups"
    echo "   ✅ All changes are preserved in git history"
    echo "   📅 Backups are 2-5 days old (Sep 30 - Oct 3)"
    echo ""
    read -p "   Proceed with deletion? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf backups/
        echo "   ✅ Deleted backups/ directory"
        echo "   💾 Reclaimed ~$BACKUP_SIZE disk space"
    else
        echo "   ⏭️  Skipped backups/ deletion"
    fi
else
    echo "🗑️  Phase 2: Delete backups/ directory"
    echo "   ℹ️  backups/ directory not found (already cleaned?)"
fi
echo ""

# Phase 3: Clean backup files in active code tree
echo "🧹 Phase 3: Clean backup files in active code"
if [ "$ACTIVE_BACKUPS" -gt 0 ]; then
    echo "   Found $ACTIVE_BACKUPS backup files in code/crates/"
    echo ""
    echo "   Files to delete:"
    find code/crates -name "*.backup-*" -o -name "*.pedantic_backup" -o -name "*.old" 2>/dev/null | head -5
    if [ "$ACTIVE_BACKUPS" -gt 5 ]; then
        echo "   ... and $((ACTIVE_BACKUPS - 5)) more"
    fi
    echo ""
    read -p "   Delete these backup files? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        find code/crates -name "*.backup-*" -delete 2>/dev/null || true
        find code/crates -name "*.pedantic_backup" -delete 2>/dev/null || true
        find code/crates -name "*.old" -delete 2>/dev/null || true
        echo "   ✅ Deleted $ACTIVE_BACKUPS backup files"
    else
        echo "   ⏭️  Skipped backup file deletion"
    fi
else
    echo "   ℹ️  No backup files found in active code tree"
fi
echo ""

# Phase 4: Preserve archive/ directory
echo "📚 Phase 4: Preserve archive/ directory"
if [ -d "archive" ]; then
    echo "   ✅ Keeping archive/ ($ARCHIVE_SIZE) - fossil record"
    echo "   ℹ️  Contains documentation history (no code files)"
else
    echo "   ℹ️  archive/ directory not found"
fi
echo ""

# Summary
echo "🎉 Cleanup Complete!"
echo "==================="
echo ""
echo "✅ Preserved:"
echo "   - archive/ directory (documentation fossil record)"
echo "   - backup.rs files (actual functionality, not backups)"
echo "   - All git history"
echo ""
echo "🗑️  Cleaned:"
if [ "$BACKUP_SIZE" != "0" ]; then
    echo "   - backups/ directory (~$BACKUP_SIZE)"
fi
if [ "$ACTIVE_BACKUPS" -gt 0 ]; then
    echo "   - Old backup files in code tree (~$ACTIVE_BACKUPS files)"
fi
echo ""
echo "💡 Next Steps (IMPORTANT):"
echo "   1. Verify build:  cargo build --workspace"
echo "   2. Verify tests:  cargo test --workspace"
echo "   3. Verify check:  cargo check --workspace"
echo "   4. If all pass, commit:"
echo "      git add -A"
echo "      git commit -m 'chore: cleanup old backup code'"
echo ""
echo "📊 Disk Space Check:"
df -h . | grep -E "Filesystem|/$" || df -h . | tail -1
echo ""

