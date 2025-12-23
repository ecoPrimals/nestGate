#!/usr/bin/env python3
"""
Sleep Elimination Tool - Replace timing assumptions with event-driven patterns
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

def find_sleep_patterns(file_path: Path) -> List[Tuple[int, str]]:
    """Find all sleep patterns in a file."""
    patterns = []
    with open(file_path, 'r') as f:
        for i, line in enumerate(f, 1):
            if 'sleep' in line.lower() and not line.strip().startswith('//'):
                patterns.append((i, line.rstrip()))
    return patterns

def analyze_sleep_usage(test_dir: Path):
    """Analyze all sleep usage in tests."""
    total_sleeps = 0
    files_with_sleeps = 0
    
    print("🔍 Analyzing sleep() usage in tests...\n")
    
    for rs_file in test_dir.rglob('*.rs'):
        if 'target' in str(rs_file) or 'archive' in str(rs_file):
            continue
            
        sleeps = find_sleep_patterns(rs_file)
        if sleeps:
            files_with_sleeps += 1
            total_sleeps += len(sleeps)
            print(f"📄 {rs_file.relative_to(test_dir.parent)}")
            for line_no, line in sleeps[:3]:  # Show first 3
                print(f"   L{line_no}: {line.strip()}")
            if len(sleeps) > 3:
                print(f"   ... and {len(sleeps) - 3} more")
            print()
    
    print(f"📊 Summary:")
    print(f"   Files with sleep(): {files_with_sleeps}")
    print(f"   Total sleep() calls: {total_sleeps}")
    print()
    
    return files_with_sleeps, total_sleeps

def generate_replacement_patterns():
    """Generate modern replacement patterns."""
    patterns = {
        'sleep_with_assert': {
            'pattern': r'tokio::time::sleep.*\n\s+assert',
            'replacement': '''// Use event-driven coordination instead
let (tx, rx) = tokio::sync::oneshot::channel();
// Signal when ready, then assert
rx.await.unwrap();
assert''',
            'note': 'Replace timing assumption with event signaling'
        },
        'sleep_between_operations': {
            'pattern': r'\.await;\n\s+tokio::time::sleep',
            'replacement': '''
.await;
// Use notification instead of sleep
notify.notified().await;''',
            'note': 'Use Notify for synchronization'
        },
    }
    
    print("💡 Modern Replacement Patterns:\n")
    print("=" * 60)
    print()
    
    print("❌ BAD - Timing Assumptions:")
    print("""```rust
tokio::time::sleep(Duration::from_millis(100)).await;
assert!(condition); // May pass due to timing luck
```
""")
    
    print("✅ GOOD - Event-Driven:")
    print("""```rust
let ready = Arc::new(Notify::new());
let ready_clone = ready.clone();

tokio::spawn(async move {
    // Do work
    ready_clone.notify_one(); // Signal when actually ready
});

ready.notified().await; // Wait for real event
assert!(condition); // Now testing actual correctness
```
""")
    
    print("=" * 60)
    print()

def main():
    repo_root = Path(__file__).parent.parent  # Go up from tools/ to repo root
    test_dir = repo_root / 'tests'
    
    if not test_dir.exists():
        print(f"❌ Test directory not found: {test_dir}")
        sys.exit(1)
    
    # Analyze
    files, total = analyze_sleep_usage(test_dir)
    
    # Show patterns
    generate_replacement_patterns()
    
    print(f"🎯 Next Steps:")
    print(f"   1. Start with files having most sleeps")
    print(f"   2. Replace each sleep with proper sync primitive")
    print(f"   3. Test under stress to verify no race conditions")
    print(f"   4. Document patterns for future reference")
    print()
    print(f"⏱️  Estimated time: {files * 10} minutes (10 min/file average)")

if __name__ == '__main__':
    main()

