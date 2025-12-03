use crate::error::{AppError, AppResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

pub struct SecurityManager {
    rate_limiter: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    anomaly_detector: Arc<RwLock<HashMap<String, UserProfile>>>,
}

#[derive(Clone, Debug)]
struct RateLimitEntry {
    count: usize,
    reset_at: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct UserProfile {
    average_tx_size: f64,
    average_tx_frequency: f64,
    last_tx: Option<chrono::DateTime<Utc>>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            anomaly_detector: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check rate limit for IP
    pub async fn check_rate_limit(
        &self,
        ip: &str,
        limit_per_minute: usize,
    ) -> AppResult<()> {
        let mut limiter = self.rate_limiter.write().await;
        let now = Utc::now();

        let entry = limiter.entry(ip.to_string()).or_insert_with(|| RateLimitEntry {
            count: 0,
            reset_at: now + chrono::Duration::minutes(1),
        });

        if now >= entry.reset_at {
            entry.count = 0;
            entry.reset_at = now + chrono::Duration::minutes(1);
        }

        entry.count += 1;

        if entry.count > limit_per_minute {
            return Err(AppError::RateLimitExceeded);
        }

        Ok(())
    }

    /// Detect anomalous spending patterns
    pub async fn detect_anomaly(
        &self,
        user_wallet: &str,
        transaction_amount: u64,
        threshold_multiplier: f64,
    ) -> AppResult<bool> {
        let mut detector = self.anomaly_detector.write().await;
        let profile = detector.entry(user_wallet.to_string())
            .or_insert_with(|| UserProfile {
                average_tx_size: transaction_amount as f64,
                average_tx_frequency: 1.0,
                last_tx: Some(Utc::now()),
            });

        // Update profile with exponential moving average
        let alpha = 0.3;
        profile.average_tx_size = (transaction_amount as f64 * alpha) 
            + (profile.average_tx_size * (1.0 - alpha));

        // Check if transaction is anomalous
        let is_anomalous = transaction_amount as f64 
            > (profile.average_tx_size * threshold_multiplier);

        Ok(is_anomalous)
    }

    /// Validate IP address format
    pub fn validate_ip(ip: &str) -> AppResult<()> {
        if ip.is_empty() {
            return Err(AppError::InvalidRequest("Invalid IP address".to_string()));
        }
        Ok(())
    }

    /// Generate device fingerprint
    pub fn generate_device_fingerprint(user_agent: &str, ip: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}", user_agent, ip).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Validate device fingerprint consistency
    pub fn validate_device_fingerprint(
        stored: &Option<String>,
        current: &str,
    ) -> AppResult<()> {
        match stored {
            Some(fingerprint) if fingerprint != current => {
                Err(AppError::SuspiciousActivity)
            }
            _ => Ok(()),
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiting() {
        let manager = SecurityManager::new();
        for _ in 0..5 {
            manager.check_rate_limit("192.168.1.1", 10).await.unwrap();
        }
        assert!(manager.check_rate_limit("192.168.1.1", 5).await.is_err());
    }

    #[test]
    fn test_fingerprint_generation() {
        let fp = SecurityManager::generate_device_fingerprint("Mozilla", "192.168.1.1");
        assert!(!fp.is_empty());
        assert_eq!(fp.len(), 64); // SHA256 hex
    }
}
