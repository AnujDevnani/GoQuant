use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use solana_program::system_instruction;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod ephemeral_vault {
    use super::*;

    /// Create a new ephemeral vault for a session
    pub fn create_ephemeral_vault(
        ctx: Context<CreateEphemeralVault>,
        approved_amount: u64,
        session_duration: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        require!(approved_amount > 0, VaultError::InvalidAmount);
        require!(session_duration > 0, VaultError::InvalidDuration);

        vault.user_wallet = ctx.accounts.user.key();
        vault.vault_pda = ctx.accounts.vault.key();
        vault.created_at = clock.unix_timestamp;
        vault.last_activity = clock.unix_timestamp;
        vault.session_expiry = clock.unix_timestamp + session_duration;
        vault.approved_amount = approved_amount;
        vault.used_amount = 0;
        vault.available_amount = 0;
        vault.total_deposited = 0;
        vault.is_active = true;
        vault.bump = ctx.bumps.get("vault").ok_or(VaultError::BumpNotFound)?;
        vault.version = 1;

        emit!(VaultCreated {
            user: ctx.accounts.user.key(),
            vault_pda: ctx.accounts.vault.key(),
            approved_amount,
            session_duration,
            expiry_timestamp: vault.session_expiry,
            timestamp: clock.unix_timestamp
        });

        Ok(())
    }

    /// Approve an ephemeral wallet as a delegate for trading
    pub fn approve_delegate(
        ctx: Context<ApproveDelegate>,
        delegate: Pubkey,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let delegation = &mut ctx.accounts.delegation;
        let clock = Clock::get()?;

        require!(vault.is_active, VaultError::VaultInactive);
        require!(clock.unix_timestamp < vault.session_expiry, VaultError::SessionExpired);

        delegation.vault = vault.key();
        delegation.delegate = delegate;
        delegation.approved_at = clock.unix_timestamp;
        delegation.revoked_at = None;
        delegation.is_active = true;

        emit!(DelegateApproved {
            vault: vault.key(),
            delegate,
            approved_at: clock.unix_timestamp
        });

        Ok(())
    }

    /// Auto-deposit SOL for transaction fees
    pub fn auto_deposit_for_trade(
        ctx: Context<AutoDeposit>,
        trade_fee_estimate: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        require!(vault.is_active, VaultError::VaultInactive);
        require!(clock.unix_timestamp < vault.session_expiry, VaultError::SessionExpired);

        let min_rent_exempt = 5000; // Minimum SOL for transaction fees
        let deposit_amount = trade_fee_estimate.max(min_rent_exempt);

        require!(
            ctx.accounts.user.lamports() >= deposit_amount,
            VaultError::InsufficientFunds
        );

        // Transfer SOL from user to vault
        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            deposit_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        vault.total_deposited = vault
            .total_deposited
            .checked_add(deposit_amount)
            .ok_or(VaultError::Overflow)?;
        vault.available_amount = vault
            .available_amount
            .checked_add(deposit_amount)
            .ok_or(VaultError::Overflow)?;
        vault.last_activity = clock.unix_timestamp;

        emit!(AutoDepositOccurred {
            vault: vault.key(),
            amount: deposit_amount,
            total_deposited: vault.total_deposited,
            timestamp: clock.unix_timestamp
        });

        Ok(())
    }

    /// Execute a trade using the vault funds (requires delegation)
    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        trade_amount: u64,
        fee_amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let delegation = &ctx.accounts.delegation;
        let clock = Clock::get()?;

        require!(vault.is_active, VaultError::VaultInactive);
        require!(clock.unix_timestamp < vault.session_expiry, VaultError::SessionExpired);
        require!(delegation.is_active, VaultError::DelegationNotActive);
        require!(
            delegation.delegate == ctx.accounts.ephemeral_wallet.key(),
            VaultError::InvalidDelegate
        );

        let total_cost = trade_amount
            .checked_add(fee_amount)
            .ok_or(VaultError::Overflow)?;

        require!(
            vault.available_amount >= total_cost,
            VaultError::InsufficientBalance
        );

        // Deduct from vault
        vault.available_amount = vault
            .available_amount
            .checked_sub(total_cost)
            .ok_or(VaultError::Overflow)?;
        vault.used_amount = vault
            .used_amount
            .checked_add(total_cost)
            .ok_or(VaultError::Overflow)?;
        vault.last_activity = clock.unix_timestamp;

        emit!(TradeExecuted {
            vault: vault.key(),
            delegate: delegation.delegate,
            trade_amount,
            fee_amount,
            timestamp: clock.unix_timestamp
        });

        Ok(())
    }

    /// Revoke delegation and return remaining funds
    pub fn revoke_access(ctx: Context<RevokeAccess>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let delegation = &mut ctx.accounts.delegation;
        let clock = Clock::get()?;

        require!(vault.is_active, VaultError::VaultInactive);

        delegation.is_active = false;
        delegation.revoked_at = Some(clock.unix_timestamp);

        // Return remaining funds to parent wallet
        let return_amount = vault.available_amount;
        if return_amount > 0 {
            **vault.to_account_info().lamports.borrow_mut() -= return_amount;
            **ctx.accounts.user.lamports.borrow_mut() += return_amount;

            vault.available_amount = 0;
        }

        vault.last_activity = clock.unix_timestamp;

        emit!(AccessRevoked {
            vault: vault.key(),
            delegate: delegation.delegate,
            returned_amount: return_amount,
            timestamp: clock.unix_timestamp
        });

        Ok(())
    }

    /// Cleanup expired vault and return remaining funds
    pub fn cleanup_vault(ctx: Context<CleanupVault>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let clock = Clock::get()?;

        require!(
            clock.unix_timestamp >= vault.session_expiry,
            VaultError::SessionNotExpired
        );

        let return_amount = vault.to_account_info().lamports();

        if return_amount > 0 {
            **vault.to_account_info().lamports.borrow_mut() = 0;
            **ctx.accounts.parent_wallet.lamports.borrow_mut() += return_amount;
        }

        emit!(VaultCleanedup {
            vault: vault.key(),
            returned_amount: return_amount,
            timestamp: clock.unix_timestamp
        });

        Ok(())
    }

    /// Close vault account
    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        Ok(())
    }
}

// ==================== ACCOUNTS ====================

#[derive(Accounts)]
#[instruction(approved_amount: u64, session_duration: i64)]
pub struct CreateEphemeralVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<EphemeralVault>(),
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, EphemeralVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveDelegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, EphemeralVault>,

    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<VaultDelegation>(),
        seeds = [b"delegation", vault.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, VaultDelegation>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AutoDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, EphemeralVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    /// CHECK: Ephemeral wallet address (not a signer in this context)
    pub ephemeral_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.user_wallet.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, EphemeralVault>,

    #[account(
        seeds = [b"delegation", vault.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, VaultDelegation>,
}

#[derive(Accounts)]
pub struct RevokeAccess<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, EphemeralVault>,

    #[account(
        mut,
        seeds = [b"delegation", vault.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, VaultDelegation>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CleanupVault<'info> {
    /// CHECK: Parent wallet to receive returned funds
    #[account(mut)]
    pub parent_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", parent_wallet.key().as_ref()],
        bump = vault.bump,
        close = parent_wallet
    )]
    pub vault: Account<'info, EphemeralVault>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump,
        close = user
    )]
    pub vault: Account<'info, EphemeralVault>,
}

// ==================== ACCOUNT STATES ====================

#[account]
pub struct EphemeralVault {
    pub user_wallet: Pubkey,        // Main user wallet
    pub vault_pda: Pubkey,          // PDA account address
    pub created_at: i64,            // Creation timestamp
    pub last_activity: i64,         // Last activity timestamp
    pub session_expiry: i64,        // Session expiry timestamp
    pub approved_amount: u64,       // Max SOL approved for delegation
    pub used_amount: u64,           // Amount currently used
    pub available_amount: u64,      // Free balance
    pub total_deposited: u64,       // Total deposited so far
    pub is_active: bool,            // Whether vault is active
    pub bump: u8,                   // PDA bump
    pub version: u8,                // Account version
}

#[account]
pub struct VaultDelegation {
    pub vault: Pubkey,              // Associated vault
    pub delegate: Pubkey,           // Delegated ephemeral wallet
    pub approved_at: i64,           // Approval timestamp
    pub revoked_at: Option<i64>,    // Revocation timestamp (if revoked)
    pub is_active: bool,            // Whether delegation is active
}

// ==================== EVENTS ====================

#[event]
pub struct VaultCreated {
    pub user: Pubkey,
    pub vault_pda: Pubkey,
    pub approved_amount: u64,
    pub session_duration: i64,
    pub expiry_timestamp: i64,
    pub timestamp: i64,
}

#[event]
pub struct DelegateApproved {
    pub vault: Pubkey,
    pub delegate: Pubkey,
    pub approved_at: i64,
}

#[event]
pub struct AutoDepositOccurred {
    pub vault: Pubkey,
    pub amount: u64,
    pub total_deposited: u64,
    pub timestamp: i64,
}

#[event]
pub struct TradeExecuted {
    pub vault: Pubkey,
    pub delegate: Pubkey,
    pub trade_amount: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct AccessRevoked {
    pub vault: Pubkey,
    pub delegate: Pubkey,
    pub returned_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct VaultCleanedup {
    pub vault: Pubkey,
    pub returned_amount: u64,
    pub timestamp: i64,
}

// ==================== ERRORS ====================

#[error_code]
pub enum VaultError {
    #[msg("Invalid vault amount")]
    InvalidAmount,
    #[msg("Invalid session duration")]
    InvalidDuration,
    #[msg("Bump not found in context")]
    BumpNotFound,
    #[msg("Vault is not active")]
    VaultInactive,
    #[msg("Session has expired")]
    SessionExpired,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Overflow in arithmetic")]
    Overflow,
    #[msg("Session not yet expired")]
    SessionNotExpired,
    #[msg("Delegation not active")]
    DelegationNotActive,
    #[msg("Invalid delegate")]
    InvalidDelegate,
}
