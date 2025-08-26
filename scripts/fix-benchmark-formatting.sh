#!/bin/bash

# Fix benchmark formatting issues
# This script fixes the common pattern of extra closing braces in benchmark files

echo "🔧 Fixing benchmark formatting issues..."

# Find all benchmark files
BENCH_FILES=$(find benches/ -name "*.rs" -type f)

for file in $BENCH_FILES; do
    echo "Fixing $file..."
    
    # Fix the pattern:    })
    #                        });
    # to:                });
    sed -i '/^    })$/{N;s/^    })\n        });$/    });/}' "$file"
    
    # Fix the pattern:    })
    #                        });
    # (with more spaces)
    sed -i '/^        })$/{N;s/^        })\n            });$/        });/}' "$file"
    
    # Fix the pattern with different indentation
    sed -i 's/^\( *\})$/\1/; /^$/N; s/^\( *\}\)\n\( *\}\);$/\2;/' "$file"
    
    # Remove extra closing braces at end of functions
    sed -i '/^    })$/{N;s/^    })\n        });$/    });/}' "$file"
done

echo "✅ Benchmark formatting fixes applied"

# Try to format again
echo "🎨 Running cargo fmt..."
cargo fmt --all || echo "⚠️  Some formatting issues remain - will need manual fixes"

echo "🔍 Checking compilation..."
cargo check --all --quiet && echo "✅ Compilation successful!" || echo "⚠️  Compilation issues remain" 