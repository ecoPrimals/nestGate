#!/usr/bin/env bash
# Analyze hardcoded port usage across the codebase

echo "=== HARDCODED PORT ANALYSIS ==="
echo ""

echo "📊 Common Ports Found:"
echo "  8080 (HTTP Alt):"
grep -r "\b8080\b" code/crates --include="*.rs" | wc -l

echo "  3000 (Dev Server):"
grep -r "\b3000\b" code/crates --include="*.rs" | wc -l

echo "  5432 (PostgreSQL):"
grep -r "\b5432\b" code/crates --include="*.rs" | wc -l

echo "  6379 (Redis):"
grep -r "\b6379\b" code/crates --include="*.rs" | wc -l

echo "  9090 (Prometheus):"
grep -r "\b9090\b" code/crates --include="*.rs" | wc -l

echo "  443 (HTTPS):"
grep -r "\b443\b" code/crates --include="*.rs" | wc -l

echo "  80 (HTTP):"
grep -r "\b80\b" code/crates --include="*.rs" | wc -l

echo ""
echo "📍 Hardcoded IPs Found:"
echo "  127.0.0.1:"
grep -r "127\.0\.0\.1" code/crates --include="*.rs" | wc -l

echo "  localhost:"
grep -r "localhost" code/crates --include="*.rs" | wc -l

echo ""
echo "📁 Files with Most Hardcoding:"
grep -r "\b8080\b\|\b3000\b\|\b5432\b\|127\.0\.0\.1\|localhost" code/crates --include="*.rs" -l | \
  xargs -I {} sh -c 'echo -n "{}: "; grep -c "\b8080\b\|\b3000\b\|\b5432\b\|127\.0\.0\.1\|localhost" {}' | \
  sort -t: -k2 -rn | head -20

echo ""
echo "✅ Analysis complete"

