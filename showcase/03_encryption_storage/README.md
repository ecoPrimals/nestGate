# NestGate + BearDog: Encrypted Data Storage

**Status**: 🔥 Live Demos  
**Focus**: Practical encryption scenarios with zero-knowledge storage  
**Goal**: Understand compression + encryption interactions

---

## The Fundamental Question

**When storing encrypted data, how do we handle:**
1. Compression (encrypt-then-compress doesn't work!)
2. Integrity verification (without decrypting)
3. Key management (read vs write vs move)
4. Zero-knowledge storage (friend drives, sharding)
5. Encrypted backups and replication

---

## The Crypto-Compression Stack

```
┌─────────────────────────────────────────────────────────────┐
│  APPLICATION                                                 │
│  (Plain data: genomic sequences, ML models, etc.)           │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  RHIZOCRYPT (Logical Layer)                                  │
│  • Deduplication (identical payloads)                        │
│  • Delta compression (version relationships)                 │
│  • Output: Optimized byte stream                            │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼ (optimized bytes)
┌─────────────────────────────────────────────────────────────┐
│  NESTGATE: Compress → Encrypt → Store                       │
│                                                              │
│  Step 1: COMPRESS (while data is plain)                     │
│    • Entropy analysis                                        │
│    • Format detection                                        │
│    • Adaptive compression                                    │
│    • Result: Compressed bytes (high compression ratio)      │
│                                                              │
│  Step 2: ENCRYPT (BearDog integration)                      │
│    • Request encryption from BearDog                         │
│    • Key management (read/write policies)                   │
│    • Result: Encrypted blob (high entropy)                  │
│                                                              │
│  Step 3: STORE (content-addressed)                          │
│    • Hash encrypted blob                                     │
│    • ZFS storage (NO further compression)                   │
│    • Result: Zero-knowledge storage                         │
│                                                              │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  STORAGE (ZFS)                                               │
│  • Stores encrypted blobs                                    │
│  • No compression (data is high entropy)                     │
│  • Deduplication at encrypted blob level                    │
└─────────────────────────────────────────────────────────────┘
```

### Why Compress THEN Encrypt?

```rust
// WRONG: Encrypt then compress
data → encrypt → [random high-entropy bytes] → compress → NO COMPRESSION!
// Encrypted data has maximum entropy (7.99/8.0)
// Compression ratio: 0.99:1 (FILE EXPANDS!)

// RIGHT: Compress then encrypt
data → compress → [small compressed bytes] → encrypt → [small encrypted blob]
// Compression works on plain data
// Encryption works on already-compressed data
// Result: Small encrypted blob with integrity
```

---

## Demo 1: Basic Encryption Flow

**Scenario**: Store genomic data with encryption

```bash
# Input: 100MB FASTA file (plain text)
genome.fasta → 100MB

# Step 1: NestGate compresses (while plain)
→ Zstd compression: 100MB → 300KB (333:1 ratio)

# Step 2: BearDog encrypts
→ AES-256-GCM: 300KB → 300KB + 16 bytes (auth tag)

# Step 3: Store encrypted blob
→ ZFS: 300KB encrypted blob
→ Hash: blake3(encrypted_blob)

# Final storage: 300KB (vs 100MB original)
# Privacy: ✅ Encrypted at rest
# Efficiency: ✅ 333x compression maintained
```

**Key Insight**: Compression savings are preserved!

---

## Demo 2: Friend Drive Backup (Zero-Knowledge)

**Scenario**: Back up encrypted data to friend's NestGate tower

### Your Side (Data Owner)

```rust
// You have: genomic dataset + encryption key
let data = fs::read("genome.fasta")?;  // 100MB

// Step 1: Compress (you know it's compressible)
let compressed = nestgate.compress(&data, Zstd19)?;  // 300KB

// Step 2: Encrypt with YOUR key
let encrypted = beardog.encrypt(&compressed, your_key_id).await?;  // 300KB

// Step 3: Send to friend's tower
let hash = friend_nestgate.store(encrypted).await?;

// Result:
// - Friend stores 300KB encrypted blob
// - Friend CANNOT read it (doesn't have key)
// - Friend CANNOT tell it's genomic data
// - You can retrieve later with hash
```

### Friend's Side (Storage Provider)

```rust
// Friend receives encrypted blob
let encrypted_blob = receive_from_network();

// Friend's NestGate analysis:
let analysis = analyzer.analyze(&encrypted_blob)?;
// → Entropy: 7.99 (high, looks random)
// → Format: Unknown (encrypted)
// → Compressibility: 0.0 (no patterns)

// Friend's routing decision:
// → PASSTHROUGH (no compression, it's already encrypted)
// → Store as-is: 300KB

// Friend CANNOT:
// ❌ Read the data
// ❌ Know what it is
// ❌ Compress it further
// ✅ Store it reliably
// ✅ Return it on demand
```

**Key Insight**: Zero-knowledge storage works!

---

## Demo 3: Encrypted Sharding

**Scenario**: Split encrypted data across multiple towers for redundancy

### Approach 1: Encrypt Then Shard (Simple)

```rust
// Step 1: Compress + Encrypt (as before)
let data = fs::read("genome.fasta")?;  // 100MB
let compressed = compress(&data)?;      // 300KB
let encrypted = encrypt(&compressed)?;  // 300KB encrypted blob

// Step 2: Shard encrypted blob
let shard_size = 100 * 1024;  // 100KB chunks
let shards = encrypted.chunks(shard_size).collect();
// → 3 shards: [100KB, 100KB, 100KB]

// Step 3: Store shards on different towers
westgate.store(shard_0).await?;   // Tower 1
stradgate.store(shard_1).await?;  // Tower 2
eastgate.store(shard_2).await?;   // Tower 3

// To reconstruct:
let shard_0 = westgate.retrieve(hash_0).await?;
let shard_1 = stradgate.retrieve(hash_1).await?;
let shard_2 = eastgate.retrieve(hash_2).await?;
let encrypted = [shard_0, shard_1, shard_2].concat();
let compressed = decrypt(&encrypted, key)?;
let data = decompress(&compressed)?;
```

**Properties:**
- ✅ Simple to implement
- ✅ All shards needed to reconstruct
- ⚠️ Lose 1 tower → lose all data

### Approach 2: Erasure Coding (Redundant)

```rust
use reed_solomon::Encoder;

// Step 1: Compress + Encrypt
let encrypted = compress_then_encrypt(&data)?;  // 300KB

// Step 2: Erasure coding (2-of-3 recovery)
let encoder = Encoder::new(3, 2);  // 3 shards, 2 needed
let shards = encoder.encode(&encrypted)?;
// → 3 shards: 150KB each
// → ANY 2 shards can reconstruct full data

// Step 3: Store on different towers
westgate.store(shard_0).await?;
stradgate.store(shard_1).await?;
eastgate.store(shard_2).await?;

// Reconstruct from ANY 2:
let shard_0 = westgate.retrieve(hash_0).await?;
let shard_2 = eastgate.retrieve(hash_2).await?;
let encrypted = encoder.decode(&[shard_0, shard_2])?;
```

**Properties:**
- ✅ Fault tolerant (lose 1 tower, still OK)
- ✅ Still zero-knowledge per tower
- ⚠️ Slightly larger total (150KB × 3 = 450KB)

---

## Demo 4: Key Management for Operations

**Scenario**: Different operations require different keys

### BearDog Key Policies

```rust
pub struct DataKeyPolicy {
    /// Who can read (decrypt)
    pub read: Vec<Did>,
    
    /// Who can write (encrypt new versions)
    pub write: Vec<Did>,
    
    /// Who can move/copy (no decryption needed)
    pub move_copy: Vec<Did>,
    
    /// Who can delete
    pub delete: Vec<Did>,
    
    /// Time-based access (loans, temporary shares)
    pub time_bound: Option<TimeBounds>,
}

// Example: Share genomic data with collaborator (read-only)
let policy = DataKeyPolicy {
    read: vec![owner_did, collaborator_did],
    write: vec![owner_did],  // Only owner can update
    move_copy: vec![owner_did],
    delete: vec![owner_did],
    time_bound: None,
};
```

### Operation Flows

**READ (requires decryption key):**
```rust
// User requests data
let encrypted_blob = nestgate.retrieve(hash).await?;

// Request decryption from BearDog
let decrypted = beardog.decrypt(
    encrypted_blob,
    requesting_did,
).await?;
// BearDog checks: Is requesting_did in policy.read?
// If yes: returns decryption key
// If no: rejects

let decompressed = decompress(decrypted)?;
```

**WRITE (requires encryption key):**
```rust
// User updates data
let new_data = modify_genome(&old_data);

// Compress
let compressed = compress(&new_data)?;

// Request encryption from BearDog
let encrypted = beardog.encrypt(
    compressed,
    requesting_did,
).await?;
// BearDog checks: Is requesting_did in policy.write?

let hash = nestgate.store(encrypted).await?;
```

**MOVE (no key needed!):**
```rust
// Move encrypted blob between towers
let encrypted_blob = westgate.retrieve(hash).await?;
// Blob is still encrypted, we don't decrypt

// Store on different tower
let new_hash = eastgate.store(encrypted_blob).await?;
// eastgate stores encrypted blob as-is

// No keys involved! Just moving encrypted bytes.
```

**DELETE (policy-based):**
```rust
// Request deletion
let result = nestgate.delete(hash, requesting_did).await?;
// BearDog checks: Is requesting_did in policy.delete?
// If yes: delete blob
// If no: reject
```

---

## Demo 5: Integrity Without Decryption

**Problem**: How to verify data integrity without decrypting?

### Solution: Authenticated Encryption (AES-GCM)

```rust
// During encryption, BearDog uses AES-256-GCM
let encrypted = beardog.encrypt(data, key_id).await?;

// Encrypted blob contains:
// - Ciphertext (encrypted data)
// - Authentication tag (16 bytes)
// - Nonce (12 bytes)

pub struct EncryptedBlob {
    ciphertext: Vec<u8>,
    auth_tag: [u8; 16],
    nonce: [u8; 12],
}

// To verify integrity WITHOUT decrypting:
let is_valid = beardog.verify_integrity(&encrypted_blob).await?;
// Checks auth tag against ciphertext + nonce
// Returns: true/false
// Does NOT decrypt!
```

### Practical Use: Verify Before Decrypting

```rust
// Retrieve encrypted blob from untrusted tower
let encrypted_blob = untrusted_tower.retrieve(hash).await?;

// Verify integrity BEFORE decrypting (fast!)
let is_valid = beardog.verify_integrity(&encrypted_blob).await?;

if !is_valid {
    return Err("Data corrupted or tampered!");
}

// Only decrypt if integrity confirmed
let decrypted = beardog.decrypt(&encrypted_blob, key).await?;
```

---

## Demo 6: Pre-Compression for Encrypted Datasets

**Question**: Should we compress before sending encrypted data to friend?

**Answer**: Already done! Compress → Encrypt workflow means:

```rust
// Your workflow:
data (100MB)
  → compress (300KB)
  → encrypt (300KB + 28 bytes)
  → send to friend (300KB)

// Friend receives: 300KB encrypted blob
// Friend stores: 300KB (as-is, no further compression)

// Network benefit: Sending 300KB instead of 100MB!
// Storage benefit: Friend stores 300KB instead of 100MB!
// Privacy benefit: Friend sees random bytes, not genomic data!
```

**Critical Insight**: Pre-compression (before encryption) is ESSENTIAL for efficiency!

---

## Demo 7: Zero-Knowledge Deduplication

**Problem**: Can NestGate deduplicate encrypted data?

### Convergent Encryption (Deterministic)

```rust
// Traditional encryption (random nonce):
encrypt(data, key) → different output each time
// Problem: Same data encrypted twice = different blobs
// No deduplication!

// Convergent encryption (deterministic):
let content_hash = blake3::hash(data);
let deterministic_key = derive_key(master_key, content_hash);
encrypt(data, deterministic_key) → same output for same data
// Benefit: Same data → same encrypted blob → deduplication!
```

**Trade-off:**
- ✅ Deduplication works (save storage)
- ⚠️ Reveals duplicate data (entropy leakage)

**Recommendation**: Use traditional (random nonce) for sensitive data, convergent for non-sensitive bulk data.

---

## Live Demo Structure

```
showcase/03_encryption_storage/
├── README.md (this file)
├── 01-basic-encryption/
│   ├── demo.sh
│   └── README.md
├── 02-friend-backup/
│   ├── demo.sh (live NestGate + BearDog)
│   └── README.md
├── 03-encrypted-sharding/
│   ├── demo-simple.sh
│   ├── demo-erasure-coding.sh
│   └── README.md
├── 04-key-management/
│   ├── demo-read-write-move.sh
│   └── README.md
├── 05-integrity-verification/
│   ├── demo.sh
│   └── README.md
├── 06-zero-knowledge-dedup/
│   ├── demo-convergent.sh
│   └── README.md
└── RUN_ALL.sh
```

---

## Key Takeaways

### 1. Compression Order Matters
- ✅ **Compress THEN Encrypt** (maintains compression ratio)
- ❌ **Encrypt THEN Compress** (compression fails, file expands)

### 2. Zero-Knowledge Storage Works
- NestGate stores encrypted blobs without knowing content
- Entropy analysis detects encrypted data → passthrough
- Friend towers can store your data securely

### 3. Key Management is Granular
- **Read**: Needs decryption key
- **Write**: Needs encryption key
- **Move**: No key needed (just moving encrypted bytes)
- **Delete**: Policy-based permission

### 4. Integrity Without Decryption
- AES-GCM provides authentication tag
- Verify integrity before decrypting
- Detect corruption/tampering without revealing data

### 5. Sharding Options
- **Simple sharding**: All pieces needed
- **Erasure coding**: Fault-tolerant (k-of-n recovery)
- Both work with encrypted data

### 6. Deduplication Trade-off
- **Random nonce**: More private, no dedup
- **Convergent**: Less private, enables dedup
- Choose based on sensitivity

---

## Next: Build Live Demos

Let's implement these scenarios with **actual BearDog + NestGate integration**, no mocks!

**Ready to proceed?**

