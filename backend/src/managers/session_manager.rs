use crate::models::*;
use crate::error::{AppError, AppResult};
use rand::Rng;
use chrono::Utc;
use uuid::Uuid;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::Aead};
use aes_gcm::KeyInit;
use sha2::{Sha256, Digest};
use base64::{Engine, engine::general_purpose};

pub struct SessionManager {
    encryption_key: Vec<u8>,
}

impl SessionManager {
    pub fn new(master_secret: &str) -> AppResult<Self> {
        // Derive a 32-byte key from the master secret using SHA256
        let mut hasher = Sha256::new();
        hasher.update(master_secret.as_bytes());
        let key = hasher.finalize().to_vec();

        Ok(Self {
            encryption_key: key,
        })
    }

    /// Generate a new ephemeral keypair with secure randomness
    pub fn generate_ephemeral_keypair(&self) -> AppResult<(String, String)> {
        // For local tests we avoid pulling the Solana SDK. Generate a
        // random placeholder public key string and random secret bytes.
        let mut rng = rand::thread_rng();
        let mut secret_bytes = vec![0u8; 64];
        rng.fill(&mut secret_bytes[..]);

        let public_key = format!("ephemeral_{}", uuid::Uuid::new_v4());

        // Encrypt the secret key
        let encrypted = self.encrypt_keypair(&secret_bytes)?;

        Ok((public_key, encrypted))
    }

    /// Create a new session with proper initialization
    pub async fn create_session(
        &self,
        user_wallet: &str,
        vault_address: &str,
        ip_address: &str,
        device_fingerprint: Option<String>,
        duration_secs: i64,
    ) -> AppResult<Session> {
        let (ephemeral_wallet, encrypted_keypair) = self.generate_ephemeral_keypair()?;
        let now = Utc::now();

        let session = Session {
            id: Uuid::new_v4(),
            user_wallet: user_wallet.to_string(),
            ephemeral_wallet,
            ephemeral_keypair: encrypted_keypair.into_bytes(),
            vault_address: vault_address.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::seconds(duration_secs),
            is_active: true,
            total_deposits: 0,
            total_spent: 0,
            ip_address: ip_address.to_string(),
            device_fingerprint,
        };

        Ok(session)
    }

    /// Verify session is still valid
    pub fn verify_session(&self, session: &Session, current_ip: &str) -> AppResult<()> {
        if !session.is_active {
            return Err(AppError::InvalidSession);
        }

        if Utc::now() > session.expires_at {
            return Err(AppError::SessionExpired);
        }

        // Verify IP matches (basic security)
        if session.ip_address != current_ip {
            return Err(AppError::Unauthorized);
        }

        Ok(())
    }

    /// Encrypt ephemeral keypair
    fn encrypt_keypair(&self, data: &[u8]) -> AppResult<String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.encryption_key));
        
        // Generate random nonce
        let mut rng = rand::thread_rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| AppError::CryptoError(e.to_string()))?;

        // Return nonce + ciphertext encoded in base64
        let mut encrypted = nonce_bytes.to_vec();
        encrypted.extend(ciphertext);
        
        Ok(general_purpose::STANDARD.encode(&encrypted))
    }

    /// Decrypt ephemeral keypair
    pub fn decrypt_keypair(&self, encrypted: &str) -> AppResult<Vec<u8>> {
        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| AppError::CryptoError(e.to_string()))?;

        if encrypted_bytes.len() < 12 {
            return Err(AppError::CryptoError("Invalid encrypted data".to_string()));
        }

        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.encryption_key));

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::CryptoError(e.to_string()))
    }

    /// Revoke a session
    pub fn revoke_session(&self, session: &mut Session) {
        session.is_active = false;
    }

    /// Check if session is near expiry (within 5 minutes)
    pub fn is_near_expiry(&self, session: &Session) -> bool {
        let time_until_expiry = session.expires_at - Utc::now();
        time_until_expiry.num_seconds() < 300 // 5 minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_generation() {
        let manager = SessionManager::new("test-secret").unwrap();
        let (pub_key, _encrypted) = manager.generate_ephemeral_keypair().unwrap();
        assert!(!pub_key.is_empty());
    }

    #[test]
    fn test_encryption_decryption() {
        let manager = SessionManager::new("test-secret").unwrap();
        let original = b"test data";
        let encrypted = manager.encrypt_keypair(original).unwrap();
        let decrypted = manager.decrypt_keypair(&encrypted).unwrap();
        assert_eq!(original, decrypted.as_slice());
    }
}
