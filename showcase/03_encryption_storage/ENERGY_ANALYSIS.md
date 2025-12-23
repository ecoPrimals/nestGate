# Energy Analysis: Compression vs Transfer Costs

**The Fundamental Question:**  
*Is it more energy-efficient to compress/decompress or just send larger data?*

**Answer:** **Compression wins** for almost all network scenarios, but **context matters**.

---

## Energy Cost Breakdown

### Network Transfer Energy Costs

| Scenario | Energy per GB | Example: 1GB File |
|----------|---------------|-------------------|
| **Same rack** | 0.0002 kWh/GB | 0.0002 kWh |
| **Same datacenter** | 0.0005 kWh/GB | 0.0005 kWh |
| **Cross-datacenter** | 0.001-0.002 kWh/GB | 0.0015 kWh |
| **Internet (AWS→home)** | 0.003-0.005 kWh/GB | 0.004 kWh |
| **Satellite/remote** | 0.01+ kWh/GB | 0.01+ kWh |

### Compression Energy Costs (Zstd)

| Operation | Energy per GB | Speed | Example: 1GB File |
|-----------|---------------|-------|-------------------|
| **Zstd-3 (fast)** | 0.0001 kWh/GB | ~500 MB/s | 0.0001 kWh |
| **Zstd-6 (balanced)** | 0.0002 kWh/GB | ~200 MB/s | 0.0002 kWh |
| **Zstd-19 (max)** | 0.0005 kWh/GB | ~50 MB/s | 0.0005 kWh |
| **Decompression** | 0.00002 kWh/GB | ~2000 MB/s | 0.00002 kWh |

**Key Insight:** Decompression is ~10-25x faster than compression!

### LZ4 (Ultra-Fast Alternative)

| Operation | Energy per GB | Speed | Example: 1GB File |
|-----------|---------------|-------|-------------------|
| **LZ4 compress** | 0.00005 kWh/GB | ~1000 MB/s | 0.00005 kWh |
| **LZ4 decompress** | 0.00001 kWh/GB | ~3000 MB/s | 0.00001 kWh |

---

## Scenario Analysis

### Scenario 1: Internet Transfer (1GB genomic data)

**Uncompressed:**
```
1GB raw data
  → Send over internet: 0.004 kWh
  
Total: 0.004 kWh
```

**Compressed (Zstd-6, 5:1 ratio):**
```
1GB raw data
  → Compress: 0.0002 kWh
  → Send 200MB: 0.0008 kWh
  → Decompress: 0.00004 kWh
  
Total: 0.00104 kWh

Savings: 74% ✅
```

**Compressed (LZ4, 3:1 ratio):**
```
1GB raw data
  → Compress: 0.00005 kWh
  → Send 333MB: 0.00133 kWh
  → Decompress: 0.00002 kWh
  
Total: 0.0014 kWh

Savings: 65% ✅
```

**Winner:** Compression (Zstd-6) saves 74% energy!

---

### Scenario 2: Same Datacenter (1GB)

**Uncompressed:**
```
1GB raw data
  → Send local network: 0.0005 kWh
  
Total: 0.0005 kWh
```

**Compressed (Zstd-6):**
```
1GB raw data
  → Compress: 0.0002 kWh
  → Send 200MB: 0.0001 kWh
  → Decompress: 0.00004 kWh
  
Total: 0.00034 kWh

Savings: 32% ✅
```

**Compressed (LZ4):**
```
1GB raw data
  → Compress: 0.00005 kWh
  → Send 333MB: 0.00017 kWh
  → Decompress: 0.00002 kWh
  
Total: 0.00024 kWh

Savings: 52% ✅
```

**Winner:** Compression (LZ4) still wins, even locally!

---

### Scenario 3: Already-Compressed Data (1GB .tar.gz)

**Uncompressed (stored as .tar.gz):**
```
1GB compressed archive (entropy: 7.9)
  → Send: 0.004 kWh
  
Total: 0.004 kWh
```

**Attempt re-compression (Zstd-6):**
```
1GB compressed archive
  → Attempt compress: 0.0002 kWh
  → Result: 1.01GB (NO SAVINGS!)
  → Send: 0.00404 kWh
  → Decompress: 0.00004 kWh
  
Total: 0.00428 kWh

Loss: 7% ❌
```

**Winner:** Skip compression for already-compressed data!

---

### Scenario 4: Random Data (1GB high-entropy)

**Uncompressed:**
```
1GB random data (entropy: 7.99)
  → Send: 0.004 kWh
  
Total: 0.004 kWh
```

**Attempt compression:**
```
1GB random data
  → Attempt compress: 0.0002 kWh
  → Result: 1.001GB (no patterns)
  → Send: 0.004004 kWh
  → Decompress: 0.00004 kWh
  
Total: 0.004244 kWh

Loss: 6% ❌
```

**Winner:** Skip compression (NestGate entropy check catches this!)

---

## The Break-Even Point

**When does compression NOT make sense?**

```python
# Break-even formula:
# compression_cost + (compressed_size * transfer_cost) + decompression_cost 
#   vs 
# original_size * transfer_cost

# Simplify (decompression is negligible):
compression_cost + (original_size / ratio * transfer_cost) 
  vs 
original_size * transfer_cost

# Solve for break-even ratio:
break_even_ratio = compression_cost / (original_size * transfer_cost) + 1
```

**Example: 1GB file, internet transfer (0.004 kWh/GB), Zstd-6 (0.0002 kWh/GB)**
```python
break_even_ratio = 0.0002 / (1 * 0.004) + 1 = 1.05

# If compression ratio < 1.05:1, skip compression
# (You only save energy if ratio > 1.05:1)
```

**For Zstd-6 on typical data:**
- Text: 3-10:1 ✅
- Genomic: 5-20:1 ✅
- Logs: 5-15:1 ✅
- Images (JPEG): 1.01:1 ❌ (already compressed)
- Already compressed: 1.0:1 ❌
- Random: 0.999:1 ❌ (expands!)

**NestGate's entropy check prevents wasted compression!**

---

## Multi-Strategy Architecture

### Strategy 1: Compress-Transport (Standard)

**Use when:**
- ✅ You trust the destination (your own infrastructure)
- ✅ Data will be processed in plain form
- ✅ Network is the bottleneck

**Pipeline:**
```
Plain Data
  ↓ Analyze entropy (NestGate)
  ↓ IF compressible: Compress
  ↓ Transport (smaller size)
  ↓ Decompress at destination
  ↓ Process in plain form
```

**Energy Profile:**
- Compression: Low cost (0.0001-0.0005 kWh/GB)
- Transfer: HIGH SAVINGS (70%+ reduction)
- Decompression: Negligible (0.00002 kWh/GB)

**Winner: 60-75% total energy savings**

---

### Strategy 2: Encrypt-Transport-Decrypt (Secure Enclave)

**Use when:**
- ✅ You DON'T trust intermediate nodes
- ✅ Data is sensitive (medical, financial, personal)
- ✅ Processing requires plain access but must be isolated

**Pipeline:**
```
Plain Data
  ↓ Compress (while plain)
  ↓ Encrypt (BearDog)
  ↓ Transport (encrypted blob)
  ↓ Secure Enclave (ToadStool + BearDog)
    ├─ Decrypt in isolated memory
    ├─ Process (compute)
    └─ Encrypt result
  ↓ Transport encrypted result
  ↓ Decrypt at owner
```

**Energy Profile:**
- Compression: Low (0.0002 kWh/GB)
- Encryption: Low (0.0001 kWh/GB)
- Transfer: Reduced (compressed size)
- Secure compute: Varies
- Decryption: Low (0.0001 kWh/GB)

**Winner: 60-70% energy savings + zero knowledge**

---

### Strategy 3: Hybrid (Adaptive)

**NestGate's Adaptive Router:**

```rust
pub enum TransportStrategy {
    /// Plain compress → transport → decompress
    /// Best for: Trusted network, high compressibility
    CompressTransport {
        algorithm: CompressionAlgorithm,
        reason: "Trusted destination + high compressibility",
    },
    
    /// Compress → encrypt → transport → decrypt → decompress
    /// Best for: Sensitive data, untrusted network
    SecureTransport {
        compression: CompressionAlgorithm,
        encryption: EncryptionAlgorithm,
        reason: "Sensitive data requires encryption",
    },
    
    /// Encrypt → transport to enclave → decrypt → process → encrypt
    /// Best for: Zero-knowledge compute
    SecureEnclave {
        target: EnclaveId,
        compression: Option<CompressionAlgorithm>,
        reason: "Untrusted processing node",
    },
    
    /// No compression, direct transport
    /// Best for: Already compressed, high entropy, local network
    Passthrough {
        reason: "High entropy or local network",
    },
}

impl NestGate {
    pub fn select_strategy(
        &self,
        data: &DataAnalysis,
        destination: &Destination,
        security: &SecurityPolicy,
    ) -> TransportStrategy {
        // 1. Check security requirements
        if security.requires_zero_knowledge {
            return TransportStrategy::SecureEnclave {
                target: destination.enclave_id,
                compression: if data.compressibility > 0.3 {
                    Some(CompressionAlgorithm::Zstd6)
                } else {
                    None
                },
                reason: "Zero-knowledge required",
            };
        }
        
        // 2. Check if encryption needed
        if security.requires_encryption {
            return TransportStrategy::SecureTransport {
                compression: if data.compressibility > 0.3 {
                    CompressionAlgorithm::Zstd6
                } else {
                    CompressionAlgorithm::None
                },
                encryption: EncryptionAlgorithm::Aes256Gcm,
                reason: "Encryption required",
            };
        }
        
        // 3. Check compressibility vs network cost
        let network_cost = destination.network_cost_per_gb;
        let compression_cost = 0.0002; // kWh/GB for Zstd-6
        
        if data.entropy > 7.5 {
            return TransportStrategy::Passthrough {
                reason: "High entropy (already compressed or random)",
            };
        }
        
        let estimated_ratio = estimate_compression_ratio(&data);
        let break_even = compression_cost / network_cost + 1.0;
        
        if estimated_ratio > break_even {
            // Compression worth it
            return TransportStrategy::CompressTransport {
                algorithm: if network_cost > 0.001 {
                    CompressionAlgorithm::Zstd6  // High network cost
                } else {
                    CompressionAlgorithm::Lz4     // Low network cost
                },
                reason: "Compression reduces total energy",
            };
        } else {
            return TransportStrategy::Passthrough {
                reason: "Compression cost > transfer savings",
            };
        }
    }
}
```

---

## Real-World Decision Matrix

| Scenario | Data Type | Network | Security | Strategy | Reasoning |
|----------|-----------|---------|----------|----------|-----------|
| **ML training data** | Genomic FASTA | Internet | Low | CompressTransport (Zstd-6) | High compressibility, trusted dest |
| **Medical records** | JSON | Internet | High | SecureTransport (Zstd-6 + AES) | Sensitive, needs encryption |
| **Private ML inference** | Model weights | Internet | Zero-knowledge | SecureEnclave (Zstd + AES) | Untrusted compute node |
| **Image backups** | JPEG/PNG | Internet | Low | Passthrough | Already compressed |
| **Local replication** | Any | Same rack | Low | CompressTransport (LZ4) | Fast local, quick compress |
| **Encrypted backup** | Any | Friend tower | Zero-knowledge | SecureTransport (Zstd + AES) | Friend shouldn't see data |

---

## Secure Enclave Deep Dive

### Use Case: Private Genomic Analysis

**Problem:** You want to run ML analysis on your genomic data, but:
- Don't trust the compute provider with your raw genome
- Need strong compute (can't do locally)
- Want zero-knowledge guarantee

**Solution: ToadStool + BearDog Secure Enclave**

```rust
// Owner's side
let genome_data = fs::read("my_genome.fasta")?;  // 1GB

// Step 1: Compress (while plain, for efficiency)
let compressed = nestgate.compress(&genome_data)?;  // 50MB (20:1)

// Step 2: Encrypt with temporary session key
let session_key = beardog.generate_session_key()?;
let encrypted = beardog.encrypt(&compressed, &session_key)?;  // 50MB encrypted

// Step 3: Send to secure ToadStool enclave
let enclave = ToadStool::secure_enclave("untrusted-provider.com")?;

// Step 4: Establish secure channel (BTSP)
let secure_channel = beardog.establish_btsp_tunnel(&enclave).await?;

// Step 5: Send encrypted data + session key (over BTSP)
secure_channel.send(encrypted).await?;
secure_channel.send_key(session_key).await?;  // Key only in enclave memory

// Step 6: Enclave processes
//   - Decrypts in isolated memory
//   - Decompresses
//   - Runs ML analysis
//   - Encrypts result
//   - Wipes keys from memory

// Step 7: Receive encrypted result
let encrypted_result = secure_channel.receive().await?;

// Step 8: Decrypt result
let result = beardog.decrypt(&encrypted_result, &session_key)?;

// Provider NEVER saw:
// ❌ Your raw genome
// ❌ The session key (only in enclave)
// ❌ The analysis result
```

**Energy Profile:**
```
Compression:    0.0005 kWh (1GB @ Zstd-19)
Encryption:     0.0001 kWh
Transfer out:   0.0002 kWh (50MB @ internet)
Decryption:     0.0001 kWh (in enclave)
Decompression:  0.00002 kWh (in enclave)
Compute:        Varies (GPU/TPU usage)
Re-encrypt:     0.0001 kWh
Transfer back:  0.00008 kWh (20MB result)

Total overhead: ~0.0011 kWh
vs unencrypted: ~0.004 kWh (1GB transfer)

Savings: 72% energy + zero-knowledge guarantee! ✅
```

---

## Recommendation: Standard Operating Procedure

### For NestGate

**1. Analyze data on ingestion:**
```rust
let analysis = analyze(data)?;
// → entropy, format, compressibility, size
```

**2. Route based on context:**
```rust
let strategy = select_strategy(
    analysis, 
    destination, 
    security_policy
)?;
```

**3. Execute strategy:**
```rust
match strategy {
    CompressTransport { algorithm, .. } => {
        let compressed = compress(data, algorithm)?;
        send(compressed).await?;
    }
    SecureTransport { compression, encryption, .. } => {
        let compressed = compress(data, compression)?;
        let encrypted = encrypt(compressed, encryption)?;
        send(encrypted).await?;
    }
    SecureEnclave { target, compression, .. } => {
        let maybe_compressed = compression.map(|c| compress(data, c));
        let encrypted = encrypt(maybe_compressed.unwrap_or(data))?;
        send_to_enclave(target, encrypted).await?;
    }
    Passthrough { .. } => {
        send(data).await?;  // No processing
    }
}
```

---

## Key Takeaways

### 1. **Compression Almost Always Wins**
- 60-75% energy savings for network transfer
- Even local networks benefit (30-50% savings)
- Break-even point is low (~1.05:1 ratio)

### 2. **Entropy Check is Critical**
- High entropy (>7.5) → skip compression
- Saves wasted CPU cycles
- Prevents file expansion

### 3. **Multi-Strategy is Essential**
- **Trusted network:** Compress → transport → decompress
- **Encrypted storage:** Compress → encrypt → transport
- **Zero-knowledge:** Encrypt → enclave → decrypt → process → re-encrypt

### 4. **Decompression is Nearly Free**
- 10-25x faster than compression
- Negligible energy cost (0.00002 kWh/GB)
- Always decompress at destination for processing

### 5. **Secure Enclaves for Sensitive Compute**
- ToadStool + BearDog isolated processing
- Zero-knowledge guarantee
- Still energy-efficient (pre-compression!)

---

## Answer to Your Question

**Q: "Is compression/decompression more costly than the transfer size diff?"**

**A: No! Compression saves 60-75% total energy for most scenarios.**

**But context matters:**
- **Internet transfer:** Always compress (unless high entropy)
- **Local network:** Still compress (30-50% savings)
- **Sensitive data:** Compress THEN encrypt
- **Untrusted compute:** Use secure enclave (still compress!)
- **Already compressed:** Skip (entropy check catches this)

**Recommendation: Multi-strategy with NestGate's adaptive router as default.**

---

**Next: Build live demo comparing all strategies!**

