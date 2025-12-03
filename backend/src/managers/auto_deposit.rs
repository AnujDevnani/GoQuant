use crate::error::{AppError, AppResult};

pub struct AutoDepositCalculator {
    base_fee: u64,
    priority_fee_multiplier: f64,
}

impl AutoDepositCalculator {
    pub fn new(base_fee: u64) -> Self {
        Self {
            base_fee,
            priority_fee_multiplier: 1.5,
        }
    }

    /// Calculate required SOL for a trade with estimated fee
    pub fn calculate_deposit_amount(&self, trade_fee_estimate: u64) -> AppResult<u64> {
        let buffer = (trade_fee_estimate as f64 * self.priority_fee_multiplier) as u64;
        let total = trade_fee_estimate
            .checked_add(buffer)
            .ok_or(AppError::InsufficientFunds)?;

        Ok(total)
    }

    /// Calculate if vault needs top-up
    pub fn should_topup(&self, current_balance: u64, pending_operations: usize) -> bool {
        let expected_cost = self.base_fee * (pending_operations as u64);
        current_balance < expected_cost * 2 // Keep 2x buffer
    }

    /// Calculate optimal deposit to bring balance to safe level
    pub fn calculate_optimal_topup(&self, current_balance: u64, pending_operations: usize) -> u64 {
        let target_balance = self.base_fee * (pending_operations as u64) * 3;
        if current_balance < target_balance {
            target_balance - current_balance
        } else {
            0
        }
    }

    /// Estimate transaction fee for different trade sizes
    pub fn estimate_trade_fee(&self, trade_size: u64) -> u64 {
        let base = 5000u64; // Base fee in lamports
        let size_fee = (trade_size / 1_000_000).max(1) * 100; // Variable based on size
        base.saturating_add(size_fee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_calculation() {
        let calc = AutoDepositCalculator::new(5000);
        let amount = calc.calculate_deposit_amount(10000).unwrap();
        assert!(amount > 10000);
    }

    #[test]
    fn test_topup_logic() {
        let calc = AutoDepositCalculator::new(5000);
        assert!(calc.should_topup(5000, 10));
        assert!(!calc.should_topup(500000, 10));
    }
}
