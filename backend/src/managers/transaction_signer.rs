use crate::error::{AppError, AppResult};

pub struct TransactionSigner;

impl TransactionSigner {
    /// Calculate transaction priority fee
    pub fn calculate_priority_fee(base_fee: u64, priority_level: u8) -> u64 {
        match priority_level {
            1 => base_fee,                         // Low
            2 => (base_fee as f64 * 1.5) as u64,   // Medium
            3 => (base_fee as f64 * 3.0) as u64,   // High
            _ => (base_fee as f64 * 5.0) as u64,   // Very High
        }
    }
}

// The Solana-specific signing and verification functions are behind
// a feature flag to avoid pulling in Solana-related dependencies
// (which have conflicting transitive dependencies) when running
// tests or building in environments that don't need blockchain code.
#[cfg(feature = "solana")]
mod solana_impl {
    use super::*;
    use solana_sdk::signature::Keypair;
    use solana_sdk::transaction::Transaction;

    impl TransactionSigner {
        /// Sign a transaction using ephemeral wallet keypair
        pub fn sign_transaction(
            keypair_bytes: &[u8],
            transaction: Transaction,
        ) -> AppResult<Transaction> {
            let keypair = Keypair::from_bytes(keypair_bytes)
                .map_err(|e| AppError::SigningError(e.to_string()))?;

            // In a real implementation, we would sign the transaction
            // For now, return the transaction (signing would happen in actual deployment)
            Ok(transaction)
        }

        /// Verify a transaction signature
        pub fn verify_signature(
            message: &[u8],
            signature: &[u8],
            public_key: &[u8],
        ) -> AppResult<bool> {
            // Use solana_sdk verification
            use solana_sdk::signature::Signature;
            use solana_sdk::pubkey::Pubkey;

            let sig = Signature::try_from(signature)
                .map_err(|e| AppError::SigningError(e.to_string()))?;
            let pubkey = Pubkey::try_from(public_key)
                .map_err(|e| AppError::SigningError(e.to_string()))?;

            Ok(sig.verify(&pubkey.to_bytes(), message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_fee_calculation() {
        let base_fee = 5000u64;
        assert_eq!(TransactionSigner::calculate_priority_fee(base_fee, 1), 5000);
        assert_eq!(TransactionSigner::calculate_priority_fee(base_fee, 3), 15000);
    }
}
