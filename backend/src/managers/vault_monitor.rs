use crate::error::AppResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VaultMonitor {
    active_vaults: Arc<RwLock<HashMap<String, VaultState>>>,
}

#[derive(Clone, Debug)]
struct VaultState {
    vault_address: String,
    session_id: String,
    balance: u64,
    last_activity: i64,
}

impl VaultMonitor {
    pub fn new() -> Self {
        Self {
            active_vaults: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add vault to monitoring
    pub async fn add_vault(&self, vault_address: String, session_id: String, initial_balance: u64) {
        let state = VaultState {
            vault_address: vault_address.clone(),
            session_id,
            balance: initial_balance,
            last_activity: chrono::Utc::now().timestamp(),
        };
        self.active_vaults.write().await.insert(vault_address, state);
    }

    /// Update vault balance
    pub async fn update_balance(&self, vault_address: &str, balance: u64) -> AppResult<()> {
        if let Some(vault) = self.active_vaults.write().await.get_mut(vault_address) {
            vault.balance = balance;
            vault.last_activity = chrono::Utc::now().timestamp();
        }
        Ok(())
    }

    /// Get current vault balance
    pub async fn get_balance(&self, vault_address: &str) -> Option<u64> {
        self.active_vaults
            .read()
            .await
            .get(vault_address)
            .map(|v| v.balance)
    }

    /// Detect abandoned vaults (no activity for specified duration)
    pub async fn detect_abandoned_vaults(&self, inactivity_threshold_secs: i64) -> Vec<String> {
        let now = chrono::Utc::now().timestamp();
        self.active_vaults
            .read()
            .await
            .iter()
            .filter(|(_, vault)| (now - vault.last_activity) > inactivity_threshold_secs)
            .map(|(addr, _)| addr.clone())
            .collect()
    }

    /// Remove vault from monitoring
    pub async fn remove_vault(&self, vault_address: &str) -> AppResult<()> {
        self.active_vaults.write().await.remove(vault_address);
        Ok(())
    }

    /// Get all active vaults
    pub async fn get_active_vaults(&self) -> Vec<String> {
        self.active_vaults
            .read()
            .await
            .keys()
            .cloned()
            .collect()
    }
}

impl Default for VaultMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vault_monitoring() {
        let monitor = VaultMonitor::new();
        monitor.add_vault("vault1".to_string(), "session1".to_string(), 1000).await;
        
        assert_eq!(monitor.get_balance("vault1").await, Some(1000));
        
        monitor.update_balance("vault1", 500).await.unwrap();
        assert_eq!(monitor.get_balance("vault1").await, Some(500));
    }
}
