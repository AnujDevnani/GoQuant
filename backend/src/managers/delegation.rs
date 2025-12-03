use crate::models::*;
use crate::error::{AppError, AppResult};
use chrono::Utc;
use uuid::Uuid;

pub struct DelegationManager;

impl DelegationManager {
    /// Verify delegation is active for a vault
    pub fn verify_delegation(delegation: &DelegationRecord, delegate: &str) -> AppResult<()> {
        if !delegation.is_active {
            return Err(AppError::InvalidSession);
        }

        if delegation.revoked_at.is_some() {
            return Err(AppError::InvalidSession);
        }

        if delegation.delegated_to != delegate {
            return Err(AppError::Unauthorized);
        }

        Ok(())
    }

    /// Create a new delegation record
    pub fn create_delegation(
        session_id: Uuid,
        vault_address: String,
        delegated_to: String,
    ) -> DelegationRecord {
        DelegationRecord {
            id: Uuid::new_v4(),
            session_id,
            vault_address,
            delegated_to,
            approved_at: Utc::now(),
            revoked_at: None,
            is_active: true,
        }
    }

    /// Revoke a delegation
    pub fn revoke_delegation(delegation: &mut DelegationRecord) {
        delegation.is_active = false;
        delegation.revoked_at = Some(Utc::now());
    }

    /// Check if delegation needs renewal
    pub fn needs_renewal(delegation: &DelegationRecord, max_age_secs: i64) -> bool {
        let age = Utc::now()
            .signed_duration_since(delegation.approved_at)
            .num_seconds();
        age > max_age_secs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delegation() {
        let session_id = Uuid::new_v4();
        let delegation = DelegationManager::create_delegation(
            session_id,
            "vault123".to_string(),
            "delegate456".to_string(),
        );
        assert!(delegation.is_active);
        assert_eq!(delegation.session_id, session_id);
    }
}
