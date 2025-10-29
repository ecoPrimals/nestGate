#!/bin/bash
# Fix cache test compilation errors

FILE="code/crates/nestgate-core/src/cache/tests.rs"

# Backup the file
cp "$FILE" "$FILE.backup"

# Fix all cold_tier_size -> cold_tier_unlimited: None
sed -i 's/cold_tier_size: None,/cold_tier_unlimited: None,/g' "$FILE"

# Fix all ttl_seconds: NUMBER -> ttl_seconds: Some(NUMBER),
sed -i 's/ttl_seconds: \([0-9]\+\),$/ttl_seconds: Some(\1),/g' "$FILE"

# Fix all cache_dir: Some(".to_string()) -> cache_dir: Some(".to_string().into()),
sed -i 's/cache_dir: Some(\(.*\)\.to_string())/cache_dir: Some(\1.to_string().into())/g' "$FILE"

# Add missing fields after hot_tier_size for CacheConfig structs
# This is trickier, so we'll need to use a Perl one-liner for multi-line matching

perl -i -pe 'BEGIN{undef $/;} s/enabled: true,\n        hot_tier_size:/enabled: true,\n        size_bytes: 1024 * 1024, \/\/ 1MB\n        cache_type: "lru".to_string(),\n        hot_tier_size:/smg' "$FILE"

perl -i -pe 'BEGIN{undef $/;} s/cold_tier_unlimited: None,\n        ttl_seconds: Some\((\d+)\),\n        cache_dir: Some\((.*?)\),\n        policy: Some\((.*?)\),/cold_tier_unlimited: None,\n        ttl_seconds: Some($1),\n        cache_dir: Some($2),\n        policy: Some($3),\n        cache_settings: std::collections::HashMap::new(),/smg' "$FILE"

echo "Cache test fixes applied. Check $FILE for changes."

