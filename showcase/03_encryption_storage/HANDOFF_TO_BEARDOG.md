# Handoff to BearDog Team: Mixed Entropy & Encryption Showcase

**Date**: December 22, 2025  
**From**: NestGate Team  
**To**: BearDog Team  
**Topic**: Mixed entropy structures, encryption, sharding for `../beardog/showcase/`

---

## What We Learned

**Critical finding**: Order matters for encryption + compression!

**Compress THEN Encrypt**: 81.7% savings ✅  
**Encrypt THEN Compress**: File expands ❌

**Measured**: Entropy detection is key to efficient storage.

---

## The Problem BearDog Solves

**Mixed entropy scenarios**:
1. User data is compressible (entropy ~2.0)
2. After encryption, entropy → 7.99 (looks random)
3. Storage systems need to handle BOTH efficiently

**Challenge**: How to support zero-knowledge storage while maintaining energy efficiency?

**Answer**: Pre-compression + smart encryption APIs

---

## What BearDog Should Build

### `../beardog/showcase/mixed-entropy/`

**Demo 1: Compress-Then-Encrypt API**
```rust
// GOOD API: Compress before encrypting
POST /btsp/encrypt-for-storage
{
    "data": "<plain_bytes>",
    "pre_compress": true,  // ← Key flag!
    "compression": "zstd-6" | "lz4" | "auto",
    "key_id": "<did>",
}

Response:
{
    "encrypted_blob": "<base64>",
    "original_size": 439,
    "compressed_size": 80,
    "encrypted_size": 80,
    "entropy_before": 2.0,
    "entropy_after": 7.99,
    "savings_percent": 81.7,
}

// NestGate receives encrypted blob:
// - Detects entropy: 7.99
// - Decision: PASSTHROUGH (no further compression)
// - Stores efficiently!
```

**Demo 2: Key Management for Operations**
```rust
pub struct DataKeyPolicy {
    // Different operations need different permissions
    read: Vec<Did>,        // Who can decrypt
    write: Vec<Did>,       // Who can encrypt new versions
    move_copy: Vec<Did>,   // Who can move blobs (NO DECRYPTION!)
    delete: Vec<Did>,      // Who can delete
    
    // Time-bound access (loans, temporary shares)
    time_bound: Option<TimeBounds>,
}

// Example: Friend tower backup
// - Friend can MOVE encrypted blobs (no key needed!)
// - Friend CANNOT read (no decryption key)
// - Owner can retrieve anytime
```

**Demo 3: Sharding with Erasure Coding**
```rust
// BearDog coordinates sharding
POST /btsp/shard-encrypted
{
    "encrypted_blob": "<compressed+encrypted>",
    "strategy": "erasure-coding",
    "params": {
        "total_shards": 5,
        "recovery_threshold": 3,  // Any 3 of 5 can recover
    }
}

Response:
{
    "shards": [
        {"shard_id": 0, "data": "<base64>", "size": 30KB},
        {"shard_id": 1, "data": "<base64>", "size": 30KB},
        {"shard_id": 2, "data": "<base64>", "size": 30KB},
        {"shard_id": 3, "data": "<base64>", "size": 30KB},
        {"shard_id": 4, "data": "<base64>", "size": 30KB},
    ],
    "metadata": {
        "original_encrypted_size": 80KB,
        "total_sharded_size": 150KB,  // Overhead for redundancy
        "fault_tolerance": "lose_2_of_5_ok",
    }
}

// Store shards on different NestGate towers
// - Westgate: shard_0, shard_1
// - Stradgate: shard_2, shard_3
// - Eastgate: shard_4
// - Lose 1 tower? Still OK!
```

**Demo 4: Integrity Without Decryption**
```rust
// AES-GCM provides authentication tag
POST /btsp/verify-integrity
{
    "encrypted_blob": "<base64>",
    // Note: NO decryption key provided!
}

Response:
{
    "is_valid": true,
    "auth_tag_verified": true,
    "tampered": false,
    "corruption_detected": false,
}

// Use case:
// - Retrieve encrypted blob from untrusted tower
// - Verify integrity BEFORE decrypting (fast!)
// - Only decrypt if valid
```

**Demo 5: Convergent Encryption (Deduplication)**
```rust
// Trade-off: Privacy vs efficiency
POST /btsp/encrypt-convergent
{
    "data": "<plain_bytes>",
    "master_key_id": "<did>",
    "deterministic": true,  // Same data → same encrypted blob
}

Response:
{
    "encrypted_blob": "<base64>",
    "deterministic": true,
    "dedup_enabled": true,
    "privacy_note": "Reveals duplicate data to storage provider",
}

// Use case:
// - Non-sensitive bulk data (public datasets)
// - Same data encrypted twice → same blob
// - NestGate can deduplicate encrypted blobs
// - Trade-off: Reveals duplicates (entropy leakage)

// Recommendation:
// - Sensitive data: Random nonce (no dedup)
// - Bulk data: Convergent (dedup OK)
```

**Demo 6: Zero-Knowledge Transfer**
```rust
// Send encrypted data to friend's NestGate
POST /btsp/prepare-for-transfer
{
    "data": "<plain_bytes>",
    "destination": "friend-nestgate-tower",
    "zero_knowledge": true,
}

Response:
{
    "encrypted_blob": "<base64>",
    "transfer_size": 80KB,  // Pre-compressed!
    "original_size": 439KB,
    "savings": "81.7%",
    "zero_knowledge": {
        "friend_can_read": false,
        "friend_can_infer": false,
        "friend_can_compress": false,  // Already max entropy
        "friend_can_store": true,
        "friend_can_return": true,
    }
}

// Friend's NestGate:
// - Receives 80KB blob (not 439KB!)
// - Detects entropy: 7.99
// - Stores as-is (PASSTHROUGH)
// - No knowledge of content
```

---

## Key Metrics to Demonstrate

1. **Entropy Impact**
   - Plain data: 2.0-6.0 bits/byte
   - After encryption: 7.99 bits/byte
   - Storage decision: PASSTHROUGH (no wasted compression)

2. **Compression Order**
   - Compress THEN encrypt: 81.7% savings ✅
   - Encrypt THEN compress: 0% savings (file expands) ❌

3. **Key Management Overhead**
   - Move operation: 0 decryptions (just bytes)
   - Read operation: 1 decryption + 1 decompression
   - Write operation: 1 compression + 1 encryption

4. **Sharding Efficiency**
   - Simple sharding: 0% overhead, no fault tolerance
   - Erasure coding (3-of-5): 87% overhead, lose 2 OK
   - Trade-off: Redundancy vs efficiency

5. **Zero-Knowledge Guarantee**
   - Friend sees: Random bytes (entropy 7.99)
   - Friend CANNOT: Decrypt, infer format, compress further
   - Audit: All operations logged (encrypted operations only)

---

## Integration Points

**BearDog provides:**
- ✅ Pre-compression API (compress before encrypt)
- ✅ Authenticated encryption (AES-256-GCM)
- ✅ Key management (read/write/move policies)
- ✅ Sharding coordination (erasure coding)
- ✅ Integrity verification (without decryption)
- ✅ BTSP tunnels (secure key exchange)

**BearDog receives from NestGate:**
- Data analysis (entropy, format, compressibility)
- Compression recommendations (Zstd-6, LZ4, Passthrough)
- Storage receipts (Blake3 hashes)

**BearDog sends to NestGate:**
- Encrypted blobs (high entropy, 7.99)
- Sharded fragments (for distributed storage)
- Integrity proofs (auth tags)

---

## Demo Architecture

### 1. Friend Backup Flow
```
Owner → BearDog: "Prepare data for friend backup"
BearDog:
  ├─ Compress (Zstd-6: 439KB → 80KB)
  ├─ Encrypt (AES-256-GCM)
  └─ Return encrypted blob

Owner → Friend NestGate: "Store this"
Friend NestGate:
  ├─ Analyze entropy: 7.99
  ├─ Decision: PASSTHROUGH
  └─ Store 80KB encrypted blob

Friend NestGate:
  ├─ Knows: Has 80KB blob with hash X
  ├─ Doesn't know: What it is, why, who, when
  └─ Can do: Store reliably, return on demand
```

### 2. Encrypted Sharding Flow
```
Owner → BearDog: "Shard my encrypted genome for redundancy"
BearDog:
  ├─ Compress genome: 1GB → 50MB (20:1)
  ├─ Encrypt: 50MB encrypted blob
  ├─ Erasure code: 3-of-5 recovery
  └─ Create 5 shards (20MB each)

Owner → NestGate towers:
  ├─ Westgate: shards 0, 1
  ├─ Stradgate: shards 2, 3
  └─ Eastgate: shard 4

Recovery scenario:
  ├─ Westgate goes down (lost shards 0, 1)
  ├─ Retrieve: shards 2, 3, 4 (from Stradgate, Eastgate)
  └─ Reconstruct: Full 50MB encrypted blob ✅

Owner → BearDog: "Decrypt recovered data"
BearDog:
  ├─ Decrypt: 50MB → compressed
  └─ Decompress: 1GB genome ✅
```

### 3. Secure Enclave Flow (with ToadStool)
```
Owner → BearDog: "Prepare genome for private analysis"
BearDog:
  ├─ Compress: 1GB → 50MB
  └─ Encrypt: Session key for ToadStool

Owner → ToadStool: "Analyze this encrypted genome"
ToadStool → BearDog: BTSP tunnel (secure key exchange)
BearDog → ToadStool: Session key (in enclave memory only)

ToadStool:
  ├─ Decrypt in isolated memory
  ├─ Decompress: 50MB → 1GB
  ├─ Analyze (variant calling)
  ├─ Re-encrypt result
  └─ Wipe keys from memory

ToadStool → Owner: Encrypted result
Owner → BearDog: "Decrypt result"
BearDog → Owner: Analysis complete

ToadStool provider: Saw only encrypted blobs (zero-knowledge) ✅
```

---

## Why This Matters

**Problem**: Encryption makes data uncompressible, wasting storage and energy.

**Solution**: BearDog coordinates compress-before-encrypt workflow:
- ✅ 81.7% savings maintained
- ✅ Zero-knowledge storage
- ✅ Key management for different operations
- ✅ Fault-tolerant sharding

**Market**: Privacy-first storage, friend backups, distributed archives

---

## Next Steps for BearDog

1. **Build pre-compression API**
   - Accept plain data
   - Compress internally (Zstd/LZ4)
   - Encrypt compressed bytes
   - Return high-entropy blob

2. **Implement key management policies**
   - Read: Needs decryption key
   - Write: Needs encryption key
   - Move: No key needed!
   - Delete: Policy-based

3. **Add sharding coordination**
   - Simple sharding (all pieces needed)
   - Erasure coding (k-of-n recovery)
   - Distribute to NestGate towers

4. **Build integrity verification**
   - AES-GCM auth tag checking
   - Verify without decrypting
   - Detect tampering/corruption

5. **Create showcase demos**
   - Friend backup (zero-knowledge)
   - Encrypted sharding (fault-tolerance)
   - Secure enclave (with ToadStool)
   - Mixed entropy handling

---

## References from NestGate

- `showcase/03_encryption_storage/README.md` - Encryption theory
- `showcase/03_encryption_storage/ENCRYPTION_COMPRESSION_ANALYSIS.md` - Order matters!
- `showcase/03_encryption_storage/ENERGY_ANALYSIS.md` - Cost breakdown
- `showcase/03_encryption_storage/SESSION_COMPLETE_*.md` - Full results
- `specs/ADAPTIVE_COMPRESSION_ARCHITECTURE.md` - NestGate's entropy detection
- `specs/CROSS_PRIMAL_COMPRESSION_INTERACTIONS.md` - Primal composition

---

**TL;DR**: Build encryption APIs in `../beardog/showcase/mixed-entropy/` that demonstrate:
1. Compress-before-encrypt (81.7% savings)
2. Key management (read/write/move)
3. Encrypted sharding (fault tolerance)
4. Zero-knowledge transfer (friend backups)

**Critical**: BearDog handles encryption + key management + sharding coordination. NestGate handles storage + entropy detection + deduplication. Together: Efficient, private, distributed storage.

**Questions?** Ping NestGate team or reference our showcase docs.

