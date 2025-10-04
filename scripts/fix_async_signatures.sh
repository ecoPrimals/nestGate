#!/bin/bash
# Fix Async Function Signatures - October 3, 2025
# Add async keyword to functions using .await

set -e

echo "🔧 Phase 4: Fixing async function signatures..."

# Backup
BACKUP_DIR="backups/async-sig-fix-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
cp -r code/crates "$BACKUP_DIR/"
echo "✅ Backup: $BACKUP_DIR"

echo ""
echo "🔧 Analyzing functions with .await but missing async..."

# Get list of files with E0728 errors
FILES=$(cargo build 2>&1 | grep "error\[E0728\]" | grep -oP '(?<=-->).*?(?=:)' | sort -u)

echo "Files with async/await issues:"
echo "$FILES"

echo ""
echo "🔧 Applying targeted fixes..."

# Strategy: For each file, find functions containing .await and ensure they have async
# This is complex, so we'll do targeted manual fixes for known files

# Fix 1: nestgate-installer/src/download.rs
FILE="code/crates/nestgate-installer/src/download.rs"
if [ -f "$FILE" ]; then
    echo "📝 Fixing $FILE..."
    # The download_release function uses .await, should be async
    sed -i 's/pub fn download_release(/pub async fn download_release(/g' "$FILE"
    echo "   ✅ Fixed download_release"
fi

# Fix 2: nestgate-installer/src/installer.rs  
FILE="code/crates/nestgate-installer/src/installer.rs"
if [ -f "$FILE" ]; then
    echo "📝 Fixing $FILE..."
    # Multiple functions need async
    sed -i 's/pub fn update(&mut self, version: Option<String>, yes: bool) -> Result<()>/pub async fn update(\&mut self, version: Option<String>, yes: bool) -> Result<()>/g' "$FILE"
    sed -i 's/pub fn install(&mut self) -> Result<()>/pub async fn install(\&mut self) -> Result<()>/g' "$FILE"
    sed -i 's/pub fn uninstall(&self, yes: bool) -> Result<()>/pub async fn uninstall(\&self, yes: bool) -> Result<()>/g' "$FILE"
    echo "   ✅ Fixed multiple functions"
fi

# Fix 3: nestgate-network/src/service/mod.rs
FILE="code/crates/nestgate-network/src/service/mod.rs"
if [ -f "$FILE" ]; then
    echo "📝 Fixing $FILE..."
    # start function uses .await
    sed -i 's/pub fn start(&self) -> nestgate_core::Result<()>/pub async fn start(\&self) -> nestgate_core::Result<()>/g' "$FILE"
    echo "   ✅ Fixed start"
fi

# Fix 4: nestgate-network/src/api.rs
FILE="code/crates/nestgate-network/src/api.rs"
if [ -f "$FILE" ]; then
    echo "📝 Fixing $FILE..."
    # Functions using .await need to be async
    sed -i 's/fn execute(&self) -> Result<serde_json::Value>/async fn execute(\&self) -> Result<serde_json::Value>/g' "$FILE"
    echo "   ✅ Fixed execute methods"
fi

echo ""
echo "🎉 Async signature fixes applied!"
echo "📊 Testing build..."

cargo build 2>&1 | grep "^error\[E" | wc -l > /tmp/error_count3.txt
ERROR_COUNT=$(cat /tmp/error_count3.txt)

echo ""
echo "📊 Build error count: $ERROR_COUNT"
echo "💾 Backup: $BACKUP_DIR"

# Show remaining async errors
REMAINING_ASYNC=$(cargo build 2>&1 | grep "error\[E0728\]" | wc -l)
echo "📊 Remaining E0728 (async) errors: $REMAINING_ASYNC"

