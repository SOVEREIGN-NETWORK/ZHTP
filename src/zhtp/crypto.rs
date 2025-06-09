use anyhow::{Result, anyhow};
use pqcrypto_dilithium::dilithium2::{
    detached_sign, keypair as dilithium_keypair, verify_detached_signature, 
    DetachedSignature, PublicKey, SecretKey,
};
use pqcrypto_kyber::kyber768;
use pqcrypto_traits::{
    sign::DetachedSignature as _,
    kem::{PublicKey as _, SecretKey as _, SharedSecret as _, Ciphertext as _},
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const KEY_ROTATION_INTERVAL: u64 = 24 * 60 * 60; // 24 hours in seconds

/// Combined post-quantum keypair
#[derive(Clone)]
pub struct Keypair {
    // Dilithium components for signatures
    pub public: PublicKey,
    secret: SecretKey,
    
    // Kyber components stored directly
    kyber_public: kyber768::PublicKey,
    kyber_secret: kyber768::SecretKey,
    
    // Key management
    pub(crate) created_at: u64,
    pub(crate) rotation_due: u64,
}

/// Key status information
#[derive(Debug, Clone)]
pub struct KeyStatus {
    pub created_at: u64,
    pub rotation_due: u64,
    pub needs_rotation: bool,
}

/// Serializable signature wrapper
#[derive(Clone, Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

/// Encapsulated key package
#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPackage {
    kyber_ciphertext: Vec<u8>,
    timestamp: u64,
}

impl Signature {
    pub fn empty() -> Self {
        Signature(Vec::new())
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        Signature(bytes)
    }
}

impl Keypair {
    /// Generate a new post-quantum keypair
    pub fn generate() -> Self {
        // Generate Dilithium keypair for signatures
        let (pk, sk) = dilithium_keypair();
        
        // Generate Kyber keypair for key encapsulation
        let (kyber_pk, kyber_sk) = kyber768::keypair();
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Keypair {
            public: pk,
            secret: sk,
            kyber_public: kyber_pk,
            kyber_secret: kyber_sk,
            created_at: now,
            rotation_due: now + KEY_ROTATION_INTERVAL,
        }
    }

    /// Sign a message using Dilithium
    pub fn sign(&self, message: &[u8]) -> Result<Signature> {
        self.check_rotation()?;
        let sig = detached_sign(message, &self.secret);
        Ok(Signature(sig.as_bytes().to_vec()))
    }

    /// Verify a Dilithium signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<bool> {
        let sig = DetachedSignature::from_bytes(&signature.0)
            .map_err(|_| anyhow!("Invalid signature format"))?;

        Ok(verify_detached_signature(&sig, message, &self.public).is_ok())
    }

    /// Encapsulate a shared secret using Kyber
    pub fn encapsulate_key(&self) -> Result<(Vec<u8>, KeyPackage)> {
        self.check_rotation()?;

        // Perform key encapsulation
        let (shared_secret, ciphertext) = kyber768::encapsulate(&self.kyber_public);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok((
            Vec::from(shared_secret.as_bytes()),
            KeyPackage {
                kyber_ciphertext: Vec::from(ciphertext.as_bytes()),
                timestamp: now,
            }
        ))
    }

    /// Decapsulate a shared secret using Kyber
    pub fn decapsulate_key(&self, package: &KeyPackage) -> Result<Vec<u8>> {
        self.check_rotation()?;

        // Convert bytes back to ciphertext
        let ct = kyber768::Ciphertext::from_bytes(&package.kyber_ciphertext)
            .map_err(|_| anyhow!("Invalid Kyber ciphertext"))?;

        // Perform decapsulation and get shared secret
        let shared_secret = kyber768::decapsulate(&ct, &self.kyber_secret);
        Ok(Vec::from(shared_secret.as_bytes()))
    }

    /// Get current key status
    pub fn get_status(&self) -> KeyStatus {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        KeyStatus {
            created_at: self.created_at,
            rotation_due: self.rotation_due,
            needs_rotation: now > self.rotation_due,
        }
    }

    /// Check if key rotation is needed
    pub fn check_rotation(&self) -> Result<()> {
        let status = self.get_status();
        if status.needs_rotation {
            Err(anyhow!("Key rotation required"))
        } else {
            Ok(())
        }
    }

    /// Create a new keypair for rotation
    pub fn rotate() -> Self {
        Self::generate()
    }

    /// Force key rotation by setting due time to now
    pub fn needs_immediate_rotation(&mut self) {
        self.rotation_due = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_lifecycle() -> Result<()> {
        let keypair = Keypair::generate();
        let message = b"Hello, quantum-resistant world!";

        let signature = keypair.sign(message)?;
        assert!(keypair.verify(message, &signature)?);

        let wrong_message = b"Hello, quantum-vulnerable world!";
        assert!(!keypair.verify(wrong_message, &signature)?);

        Ok(())
    }

    #[test]
    fn test_key_encapsulation() -> Result<()> {
        let alice_keypair = Keypair::generate();
        let bob_keypair = Keypair::generate();

        // Alice encapsulates a secret for Bob
        let (secret1, package) = bob_keypair.encapsulate_key()?;

        // Bob decapsulates the secret
        let secret2 = bob_keypair.decapsulate_key(&package)?;

        // The secrets should match
        assert_eq!(secret1, secret2);
        
        Ok(())
    }

    #[test]
    fn test_different_keypairs() -> Result<()> {
        let keypair1 = Keypair::generate();
        let keypair2 = Keypair::generate();
        let message = b"Test message";

        let signature = keypair1.sign(message)?;
        assert!(keypair1.verify(message, &signature)?);
        assert!(!keypair2.verify(message, &signature)?);

        Ok(())
    }

    #[test]
    fn test_key_rotation() -> Result<()> {
        let mut keypair = Keypair::generate();
        keypair.needs_immediate_rotation();
        
        let message = b"Test message";
        assert!(keypair.sign(message).is_err());
        
        Ok(())
    }
}
