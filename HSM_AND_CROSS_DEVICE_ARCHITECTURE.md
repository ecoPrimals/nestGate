# HSM and Cross-Device Key Architecture

## 🔐 **What is HSM (Hardware Security Module)?**

### **HSM = Dedicated Crypto Processor**
```HSM (Hardware Security Module)
├── Tamper-resistant hardware chip
├── Dedicated crypto processor
├── Secure key storage (keys never leave in plaintext)
├── Hardware-backed random number generation
└── Cryptographic operations (sign, encrypt, derive)
```

### **HSM Types**
```rust
pub enum HSMType {
    /// Dedicated network-attached HSM appliance
    NetworkHSM {
        endpoint: String,
        cluster: Vec<String>,
    },
    /// USB/PCIe HSM device
    LocalHSM {
        device_path: String,
        serial_number: String,
    },
    /// Cloud HSM service (AWS CloudHSM, Azure Dedicated HSM)
    CloudHSM {
        provider: String,
        region: String,
        instance_id: String,
    },
    /// Software HSM (for development/testing)
    SoftHSM {
        token_path: String,
        pin: String,
    },
}
```

## 🔒 **HSM Security Model**

### **Keys NEVER Leave HSM in Plaintext**
```rust
// What HSM CAN do
pub trait HSMOperations {
    /// Generate key inside HSM (key never leaves)
    async fn generate_key(&self, key_id: &str) -> Result<KeyHandle>;
    
    /// Sign data using HSM key (key never leaves)
    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Signature>;
    
    /// Derive child key inside HSM (master key never leaves)
    async fn derive_key(&self, master_key_id: &str, derivation_path: &str) -> Result<KeyHandle>;
    
    /// Encrypt data using HSM key (key never leaves)
    async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>>;
}

// What HSM CANNOT do
pub trait HSMRestrictions {
    /// ❌ Export master key in plaintext (security violation)
    async fn export_master_key(&self, key_id: &str) -> Result<Vec<u8>> {
        Err(HSMError::SecurityViolation("Master keys never leave HSM"))
    }
    
    /// ❌ Backup master key unencrypted (security violation)
    async fn backup_master_key_plaintext(&self, key_id: &str) -> Result<Vec<u8>> {
        Err(HSMError::SecurityViolation("Master keys never leave HSM"))
    }
}
```

## 📱 **Cross-Device Key Access Solutions**

### **Solution 1: Device-Specific Key Derivation**
```rust
// Your master seed stays in HSM, derive device-specific keys
pub struct CrossDeviceKeyManager {
    hsm: Arc<dyn HSMProvider>,
    master_seed_id: String,
}

impl CrossDeviceKeyManager {
    /// Derive device-specific key from master seed
    pub async fn provision_device_key(
        &self,
        device_id: &str,
        device_type: DeviceType,
        user_auth: &UserAuth,
    ) -> Result<DeviceKey> {
        // Create device-specific derivation path
        let derivation_path = format!("m/device/{}/{}", 
            device_type.to_index(), device_id);
        
        // Derive device key inside HSM (master seed never leaves)
        let device_key_handle = self.hsm.derive_key(
            &self.master_seed_id,
            &derivation_path,
        ).await?;
        
        // HSM signs the device key for local use
        let signed_device_key = self.hsm.sign_key_for_device(
            &device_key_handle,
            device_id,
            user_auth,
        ).await?;
        
        Ok(DeviceKey {
            device_id: device_id.to_string(),
            device_type,
            key_handle: device_key_handle,
            signed_key: signed_device_key,
            derivation_path,
            expires_at: Some(Utc::now() + Duration::days(30)),
        })
    }
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Phone,              // m/device/0/{device_id}
    Laptop,             // m/device/1/{device_id}
    Desktop,            // m/device/2/{device_id}
    Server,             // m/device/3/{device_id}
    IoTDevice,          // m/device/4/{device_id}
    SecondaryAuth,      // m/device/100/{device_id} (for 2FA)
}
```

### **Solution 2: HSM-Signed Portable Keys**
```rust
// HSM can sign keys for use on other devices
pub struct PortableKeyManager {
    hsm: Arc<dyn HSMProvider>,
    master_seed_id: String,
}

impl PortableKeyManager {
    /// Create portable key signed by HSM
    pub async fn create_portable_key(
        &self,
        target_device: &str,
        purpose: &str,
        duration: Duration,
        user_auth: &UserAuth,
    ) -> Result<PortableKey> {
        // Derive purpose-specific key inside HSM
        let derivation_path = format!("m/portable/{}/{}", target_device, purpose);
        let portable_key_handle = self.hsm.derive_key(
            &self.master_seed_id,
            &derivation_path,
        ).await?;
        
        // HSM signs the portable key with expiration
        let hsm_signature = self.hsm.sign_with_expiration(
            &self.master_seed_id,
            &portable_key_handle,
            duration,
            user_auth,
        ).await?;
        
        // Create portable key package
        Ok(PortableKey {
            key_data: portable_key_handle.to_exportable_format(),
            hsm_signature,
            target_device: target_device.to_string(),
            purpose: purpose.to_string(),
            expires_at: Utc::now() + duration,
            usage_restrictions: vec![
                format!("device:{}", target_device),
                format!("purpose:{}", purpose),
            ],
        })
    }
    
    /// Validate portable key on target device
    pub async fn validate_portable_key(
        &self,
        portable_key: &PortableKey,
        current_device: &str,
    ) -> Result<bool> {
        // Check expiration
        if portable_key.expires_at < Utc::now() {
            return Ok(false);
        }
        
        // Check device restriction
        if portable_key.target_device != current_device {
            return Ok(false);
        }
        
        // Verify HSM signature
        let signature_valid = self.hsm.verify_signature(
            &self.master_seed_id,
            &portable_key.key_data,
            &portable_key.hsm_signature,
        ).await?;
        
        Ok(signature_valid)
    }
}
```

### **Solution 3: Multi-Device Enrollment**
```rust
// Register multiple devices with HSM
pub struct MultiDeviceManager {
    hsm: Arc<dyn HSMProvider>,
    master_seed_id: String,
    enrolled_devices: Arc<RwLock<HashMap<String, EnrolledDevice>>>,
}

impl MultiDeviceManager {
    /// Enroll new device with HSM
    pub async fn enroll_device(
        &self,
        device_id: &str,
        device_info: &DeviceInfo,
        user_auth: &UserAuth,
    ) -> Result<DeviceEnrollmentResult> {
        // Generate device-specific key pair
        let device_key_id = format!("device-{}", device_id);
        let device_key_handle = self.hsm.generate_key(&device_key_id).await?;
        
        // Create enrollment certificate signed by master seed
        let enrollment_cert = self.hsm.sign_certificate(
            &self.master_seed_id,
            &device_key_handle,
            device_info,
            user_auth,
        ).await?;
        
        // Store device enrollment
        let enrolled_device = EnrolledDevice {
            device_id: device_id.to_string(),
            device_info: device_info.clone(),
            key_handle: device_key_handle,
            enrollment_cert,
            enrolled_at: Utc::now(),
            last_used: None,
        };
        
        self.enrolled_devices.write().await
            .insert(device_id.to_string(), enrolled_device.clone());
        
        Ok(DeviceEnrollmentResult {
            device_key_id,
            enrollment_cert: enrolled_device.enrollment_cert,
            instructions: format!("Install enrollment certificate on device {}", device_id),
        })
    }
    
    /// Use enrolled device for operations
    pub async fn use_enrolled_device(
        &self,
        device_id: &str,
        operation: &str,
        device_proof: &DeviceProof,
    ) -> Result<OperationResult> {
        // Get enrolled device
        let enrolled_devices = self.enrolled_devices.read().await;
        let device = enrolled_devices.get(device_id)
            .ok_or(HSMError::DeviceNotEnrolled)?;
        
        // Verify device proof using enrollment certificate
        let proof_valid = self.hsm.verify_device_proof(
            &device.enrollment_cert,
            device_proof,
        ).await?;
        
        if !proof_valid {
            return Err(HSMError::InvalidDeviceProof);
        }
        
        // Perform operation using device's key
        let result = self.hsm.perform_operation(
            &device.key_handle,
            operation,
        ).await?;
        
        Ok(result)
    }
}
```

## 📱 **Phone/Secondary Device Examples**

### **Example 1: Phone as Secondary Auth**
```rust
// Your phone becomes a secondary authenticator
pub async fn setup_phone_as_second_id(
    hsm: &dyn HSMProvider,
    master_seed_id: &str,
    phone_id: &str,
) -> Result<PhoneSecondaryAuth> {
    // Derive phone-specific key from master seed
    let phone_key = hsm.derive_key(
        master_seed_id,
        &format!("m/2fa/phone/{}", phone_id),
    ).await?;
    
    // Create signed authentication token for phone
    let auth_token = hsm.sign_auth_token(
        master_seed_id,
        &phone_key,
        Duration::hours(24), // Valid for 24 hours
    ).await?;
    
    Ok(PhoneSecondaryAuth {
        phone_id: phone_id.to_string(),
        auth_token,
        qr_code: generate_qr_code(&auth_token),
        setup_instructions: "Scan QR code with your phone's authenticator app".to_string(),
    })
}
```

### **Example 2: Laptop Access**
```rust
// Your laptop gets a derived key for local operations
pub async fn provision_laptop_access(
    hsm: &dyn HSMProvider,
    master_seed_id: &str,
    laptop_id: &str,
    user_auth: &UserAuth,
) -> Result<LaptopKey> {
    // Derive laptop-specific key
    let laptop_key = hsm.derive_key(
        master_seed_id,
        &format!("m/device/laptop/{}", laptop_id),
    ).await?;
    
    // Create signed key package for laptop
    let signed_key_package = hsm.create_signed_key_package(
        master_seed_id,
        &laptop_key,
        laptop_id,
        Duration::days(30), // Valid for 30 days
        user_auth,
    ).await?;
    
    Ok(LaptopKey {
        laptop_id: laptop_id.to_string(),
        key_package: signed_key_package,
        installation_script: generate_installation_script(&signed_key_package),
    })
}
```

## 🔄 **Key Recovery and Backup**

### **HSM-Safe Backup Method**
```rust
// Backup master seed using threshold cryptography
pub struct HSMBackupManager {
    hsm: Arc<dyn HSMProvider>,
    master_seed_id: String,
}

impl HSMBackupManager {
    /// Create secure backup using Shamir's Secret Sharing
    pub async fn create_secure_backup(
        &self,
        threshold: u8,
        total_shares: u8,
        user_auth: &UserAuth,
    ) -> Result<BackupShares> {
        // HSM creates secret shares (master seed never leaves HSM)
        let backup_shares = self.hsm.create_secret_shares(
            &self.master_seed_id,
            threshold,
            total_shares,
            user_auth,
        ).await?;
        
        Ok(BackupShares {
            shares: backup_shares,
            threshold,
            total_shares,
            instructions: format!(
                "Store {} shares securely. Need {} shares to recover master seed.",
                total_shares, threshold
            ),
        })
    }
    
    /// Recover master seed from backup shares
    pub async fn recover_from_backup(
        &self,
        backup_shares: Vec<BackupShare>,
        new_hsm: &dyn HSMProvider,
    ) -> Result<String> {
        // Verify we have enough shares
        if backup_shares.len() < backup_shares[0].threshold as usize {
            return Err(HSMError::InsufficientShares);
        }
        
        // Reconstruct master seed inside new HSM
        let recovered_master_seed_id = new_hsm.reconstruct_master_seed(
            &backup_shares,
        ).await?;
        
        Ok(recovered_master_seed_id)
    }
}
```

## ✅ **Summary: HSM + Cross-Device Access**

### **HSM Security Model**
- **✅ Master seed NEVER leaves HSM in plaintext**
- **✅ All operations performed inside HSM**
- **✅ Tamper-resistant hardware protection**
- **✅ Hardware-backed random number generation**

### **Cross-Device Solutions**
- **✅ Device-specific key derivation** (phone, laptop, etc.)
- **✅ HSM-signed portable keys** (time-limited)
- **✅ Multi-device enrollment** (register all your devices)
- **✅ Secondary authentication** (phone as 2FA)

### **Key Points**
1. **Master seed stays in HSM** - never moves, never exposed
2. **Devices get derived keys** - mathematically related but different
3. **HSM signs everything** - proves authenticity
4. **Time-limited access** - keys expire for security
5. **Secure backup** - using secret sharing, not plaintext

**Your master seed is truly hardware-locked, but you can still use it across all your devices through secure derivation and signing!** 