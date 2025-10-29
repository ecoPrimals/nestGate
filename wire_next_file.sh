#!/bin/bash
# Wire integration tests for a given handler file

set -e

FILE=$1

if [ -z "$FILE" ]; then
    echo "═══════════════════════════════════════════════════════"
    echo "🔌 Wire Test Integration Script"
    echo "═══════════════════════════════════════════════════════"
    echo ""
    echo "Usage: ./wire_next_file.sh <path/to/handler.rs>"
    echo ""
    echo "Example:"
    echo "  ./wire_next_file.sh code/crates/nestgate-api/src/handlers/status.rs"
    echo ""
    echo "═══════════════════════════════════════════════════════"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "❌ Error: File not found: $FILE"
    exit 1
fi

echo "═══════════════════════════════════════════════════════"
echo "🔌 Wiring Tests for: $FILE"
echo "═══════════════════════════════════════════════════════"
echo ""

# Step 1: Check if file already has #[cfg(test)]
echo "Step 1: Checking for existing test module..."
if grep -q "#\[cfg(test)\]" "$FILE"; then
    echo "✅ File already has test module"
    echo "   You can add integration tests before the closing brace"
    HAS_TESTS=true
else
    echo "⚠️  File has no test module"
    HAS_TESTS=false
fi

echo ""

# Step 2: Find handler functions
echo "Step 2: Finding handler functions..."
echo ""
HANDLERS=$(grep -n "^pub async fn" "$FILE" 2>/dev/null || echo "")

if [ -z "$HANDLERS" ]; then
    echo "⚠️  No async handler functions found"
    echo "   Looking for sync functions..."
    HANDLERS=$(grep -n "^pub fn" "$FILE" 2>/dev/null || echo "")
fi

if [ -z "$HANDLERS" ]; then
    echo "❌ No handler functions found in file"
    exit 1
fi

echo "Found handler functions:"
echo "$HANDLERS" | head -10
echo ""

# Step 3: Count handlers
HANDLER_COUNT=$(echo "$HANDLERS" | wc -l)
echo "Total handlers: $HANDLER_COUNT"
echo ""

# Step 4: Check existing test count
echo "Step 3: Checking existing tests..."
TEST_COUNT=$(grep -c "#\[test\]" "$FILE" 2>/dev/null || echo 0)
ASYNC_TEST_COUNT=$(grep -c "#\[tokio::test\]" "$FILE" 2>/dev/null || echo 0)
TOTAL_TESTS=$((TEST_COUNT + ASYNC_TEST_COUNT))

echo "Existing tests: $TOTAL_TESTS ($TEST_COUNT sync, $ASYNC_TEST_COUNT async)"
echo ""

# Step 5: Recommendations
echo "═══════════════════════════════════════════════════════"
echo "📝 RECOMMENDATIONS"
echo "═══════════════════════════════════════════════════════"
echo ""

if [ "$HAS_TESTS" = true ]; then
    echo "✅ Add integration tests to existing test module"
    echo ""
    echo "Template to add:"
    echo ""
    cat << 'EOF'
    // ==================== INTEGRATION TESTS ====================

    /// Helper to create test API state
    fn create_test_state() -> ApiState {
        ApiState::new().expect("Failed to create test state")
    }

    #[tokio::test]
    async fn test_handler_name() {
        let state = create_test_state();
        let result = handler_function(State(state)).await;
        assert!(result.is_ok());
    }
EOF
else
    echo "⚠️  Create test module first"
    echo ""
    echo "Add this at the end of $FILE:"
    echo ""
    cat << 'EOF'
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ApiState;
    use axum::extract::State;

    /// Helper to create test API state
    fn create_test_state() -> ApiState {
        ApiState::new().expect("Failed to create test state")
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_handler_basic() {
        let state = create_test_state();
        // TODO: Add handler calls here
    }
}
EOF
fi

echo ""
echo "═══════════════════════════════════════════════════════"
echo "🎯 NEXT STEPS"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "1. Open file: $FILE"
echo ""
echo "2. Add integration tests for these handlers:"
FIRST_FIVE=$(echo "$HANDLERS" | head -5 | awk -F: '{print $2}' | sed 's/pub async fn //' | sed 's/pub fn //' | awk '{print "   -", $1}')
echo "$FIRST_FIVE"
if [ $HANDLER_COUNT -gt 5 ]; then
    echo "   ... and $((HANDLER_COUNT - 5)) more"
fi
echo ""
echo "3. Run tests:"
BASENAME=$(basename "$FILE" .rs)
MODULE_PATH=$(echo "$FILE" | sed 's|code/crates/nestgate-api/src/||' | sed 's|/|::|g' | sed 's|\.rs$||')
echo "   cargo test --package nestgate-api --lib $MODULE_PATH::tests"
echo ""
echo "4. Verify coverage:"
echo "   cargo tarpaulin --files \"$FILE\""
echo ""
echo "5. Commit changes:"
echo "   git add $FILE"
echo "   git commit -m \"Wire integration tests for $(basename $FILE)\""
echo ""
echo "═══════════════════════════════════════════════════════"
echo "Example integration test pattern:"
echo "═══════════════════════════════════════════════════════"
echo ""
cat << 'EOF'
#[tokio::test]
async fn test_list_items() {
    let state = create_test_state();
    let query = ListQuery::default();
    let result = list_items(State(state), Query(query)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_item_validation() {
    let state = create_test_state();
    let invalid = CreateRequest { name: "".into() };
    let result = create_item(State(state), Json(invalid)).await;
    assert!(result.is_err(), "Empty name should fail");
}
EOF

echo ""
echo "═══════════════════════════════════════════════════════"
echo "✅ Analysis complete!"
echo "═══════════════════════════════════════════════════════"

