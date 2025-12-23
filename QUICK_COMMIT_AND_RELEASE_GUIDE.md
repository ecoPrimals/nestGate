📧 COPY/PASTE THIS TO TEAMS:

═══════════════════════════════════════════════════════════

Subject: Quick Guide - Commit, Clean Branches, & Release Binaries

Hi Team,

Quick reference for our workflow:

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ COMMIT & PUSH ALL CHANGES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

```bash
git add -A
git commit -m "Your message"
git push origin $(git branch --show-current)
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🧹 CLEAN OLD BRANCHES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

```bash
# Update remote refs
git fetch -p

# Delete local merged branches (safe)
git branch --merged | grep -v "main" | grep -v "\*" | xargs git branch -d

# Delete specific branch
git branch -d old-branch-name

# Delete remote branch
git push origin --delete old-branch-name
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📦 RELEASE BINARY (Don't Push to Repo!)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

```bash
# 1. Build release
cargo build --release  # or your build command

# 2. Create checksum
sha256sum target/release/your-binary > your-binary.sha256

# 3. Create & push tag
git tag -a v0.1.0-integration -m "Integration checkpoint"
git push origin v0.1.0-integration

# 4. Install GitHub CLI (if needed)
sudo apt install gh
gh auth login --web

# 5. Create release with binary
gh release create v0.1.0-integration \
  target/release/your-binary \
  your-binary.sha256 \
  --title "Integration Checkpoint" \
  --notes "Ready for testing" \
  --prerelease

# Share download URL with teams:
# https://github.com/your-org/your-repo/releases/tag/v0.1.0-integration
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎯 WHY THIS MATTERS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Binaries in GitHub Releases (not repo) = Clean Git history
✅ Tags = Traceable checkpoints
✅ Clean branches = Easier to navigate
✅ One-line commands = Fast workflow

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📚 FULL GUIDE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

See: QUICK_COMMIT_AND_RELEASE_GUIDE.md (in NestGate repo)
Example: https://github.com/ecoPrimals/bearDog/releases/tag/v0.9.0-integration-dec23

Questions? Ask in team chat!

🐻 ecoPrimals - Keep it clean!

═══════════════════════════════════════════════════════════

---

## 📖 DETAILED EXAMPLES

### NestGate-Specific Commands

```bash
# Build NestGate release
cargo build --release --workspace

# Main binary locations
ls -lh target/release/nestgate
ls -lh target/release/nestgate-api-server

# Create checksums
sha256sum target/release/nestgate > nestgate.sha256
sha256sum target/release/nestgate-api-server > nestgate-api-server.sha256

# Create release with both binaries
gh release create v0.1.0-audit-dec23 \
  target/release/nestgate \
  target/release/nestgate-api-server \
  nestgate.sha256 \
  nestgate-api-server.sha256 \
  --title "NestGate v0.1.0 - Audit Complete" \
  --notes "Post-audit checkpoint. See COMPREHENSIVE_AUDIT_REPORT_DEC_23_2025.md" \
  --prerelease
```

### Clean Up Old Branches

```bash
# List all branches
git branch -a

# Check which branches are safe to delete
git branch --merged

# Safe cleanup (removes local merged branches)
git fetch -p
git branch --merged | grep -v "main" | grep -v "master" | grep -v "\*" | xargs git branch -d

# Check remote branches
git branch -r

# Delete specific remote branches that are no longer needed
git push origin --delete old-feature-branch
git push origin --delete old-experiment-branch
```

### Create Integration Checkpoints

```bash
# After major work (like this audit)
git tag -a v0.1.0-audit-dec23 -m "Comprehensive audit complete, workspace cleaned"
git push origin v0.1.0-audit-dec23

# For working checkpoints
git tag -a v0.1.0-wip-fixes -m "Critical fixes in progress"
git push origin v0.1.0-wip-fixes

# List all tags
git tag -l

# Delete tag if needed
git tag -d v0.1.0-old-tag
git push origin --delete v0.1.0-old-tag
```

---

## 🔧 NESTGATE CURRENT STATUS

**Branch**: `week-1-4-production-readiness`  
**Status**: ✅ Committed and pushed  
**Latest Commit**: Comprehensive audit and workspace cleanup  

**Files**:
- `COMPREHENSIVE_AUDIT_REPORT_DEC_23_2025.md` - Full audit report
- `CRITICAL_FIXES_ACTION_PLAN.md` - 90-minute fix plan
- `WORKSPACE_CLEANUP_COMPLETE_DEC_23_2025.md` - Cleanup summary

**Next Steps**:
1. Create PR: https://github.com/ecoPrimals/nestGate/pull/new/week-1-4-production-readiness
2. Fix critical issues (build failures)
3. Create release after fixes complete

---

## 🚀 QUICK RELEASE WORKFLOW

### Step 1: Ensure Clean State
```bash
git status  # Should be clean
cargo build --release --workspace  # Must succeed
cargo test --workspace  # Should pass
```

### Step 2: Create Release Tag
```bash
VERSION="v0.1.0-dec23-fixes"
git tag -a $VERSION -m "Critical fixes applied"
git push origin $VERSION
```

### Step 3: Build & Verify Binaries
```bash
# Build
cargo build --release --workspace

# Verify binaries exist
ls -lh target/release/nestgate*

# Create checksums
cd target/release
sha256sum nestgate nestgate-api-server > nestgate-checksums.sha256
cd ../..
```

### Step 4: Create GitHub Release
```bash
gh release create $VERSION \
  target/release/nestgate \
  target/release/nestgate-api-server \
  target/release/nestgate-checksums.sha256 \
  COMPREHENSIVE_AUDIT_REPORT_DEC_23_2025.md \
  CRITICAL_FIXES_ACTION_PLAN.md \
  --title "NestGate $VERSION - Critical Fixes" \
  --notes-file CHANGELOG.md \
  --prerelease
```

### Step 5: Share with Team
```
📦 NestGate Release Available!

Version: $VERSION
Download: https://github.com/ecoPrimals/nestGate/releases/tag/$VERSION

Binaries:
- nestgate (main CLI)
- nestgate-api-server (API server)
- nestgate-checksums.sha256 (verification)

Documentation:
- Audit report included
- Fix plan included

Status: Pre-release (testing phase)
```

---

## 📋 BRANCH CLEANUP CHECKLIST

### Before Cleaning
- [ ] Ensure all important work is committed
- [ ] Verify branches are merged to main/master
- [ ] Check with team if any branches are in use
- [ ] Backup any experimental branches you want to keep

### Safe Cleanup
```bash
# 1. Update your local repo
git fetch -p

# 2. Check merged branches
git branch --merged

# 3. Delete local merged branches (safe)
git branch --merged | grep -v "main" | grep -v "master" | grep -v "\*" | xargs git branch -d

# 4. Check unmerged branches (DON'T auto-delete these)
git branch --no-merged

# 5. Manually review and delete specific old branches
git branch -d old-branch-name
```

### Remote Cleanup
```bash
# List remote branches
git branch -r

# Delete remote branches that are no longer needed
git push origin --delete old-feature-branch

# Note: Be careful with remote deletion - check with team first!
```

---

## ⚠️ IMPORTANT NOTES

### DO NOT
- ❌ Push large binaries to git repository
- ❌ Force push to main/master branches
- ❌ Delete branches without checking team
- ❌ Skip checksums on releases

### DO
- ✅ Use GitHub Releases for binaries
- ✅ Create tags for checkpoints
- ✅ Include checksums with releases
- ✅ Write clear commit messages
- ✅ Test before releasing
- ✅ Use pre-release flag for testing

---

## 🔗 USEFUL LINKS

**NestGate Repository**: https://github.com/ecoPrimals/nestGate  
**BearDog Releases**: https://github.com/ecoPrimals/bearDog/releases  
**Songbird Repository**: https://github.com/ecoPrimals/songbird  
**ToadStool Repository**: https://github.com/ecoPrimals/toadstool  

**Current Branch**: week-1-4-production-readiness  
**Create PR**: https://github.com/ecoPrimals/nestGate/pull/new/week-1-4-production-readiness

---

## 💡 TIPS

1. **Commit Often**: Small, focused commits are better than large ones
2. **Tag Milestones**: Use tags to mark important checkpoints
3. **Clean Regularly**: Delete merged branches weekly
4. **Test Releases**: Always use --prerelease for initial releases
5. **Document Changes**: Update CHANGELOG.md with each release
6. **Verify Checksums**: Always create and verify SHA256 checksums
7. **Team Communication**: Share release URLs in team chat

---

**Last Updated**: December 23, 2025  
**Status**: Ready to use  
**Maintained by**: ecoPrimals Team

🐻 Keep building sovereign, ethical AI systems!

