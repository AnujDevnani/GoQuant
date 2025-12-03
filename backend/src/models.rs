use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_wallet: String,
    pub ephemeral_wallet: String,
    pub ephemeral_keypair: Vec<u8>, // Encrypted
    pub vault_address: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub total_deposits: u64,
    pub total_spent: u64,
    pub ip_address: String,
    pub device_fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultTransaction {
    pub id: Uuid,
    pub session_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "fee")]
    Fee,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "deposit"),
            TransactionType::Trade => write!(f, "trade"),
            TransactionType::Withdrawal => write!(f, "withdrawal"),
            TransactionType::Fee => write!(f, "fee"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum TransactionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "failed")]
    Failed,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "pending"),
            TransactionStatus::Confirmed => write!(f, "confirmed"),
            TransactionStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRecord {
    pub id: Uuid,
    pub session_id: Uuid,
    pub vault_address: String,
    pub delegated_to: String,
    pub approved_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub vault_address: String,
    pub returned_amount: u64,
    pub cleaned_at: DateTime<Utc>,
    pub reason: CleanupReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CleanupReason {
    Expired,
    Revoked,
    Manual,
    AbandonedFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalytics {
    pub user_wallet: String,
    pub total_sessions: i64,
    pub active_sessions: i64,
    pub total_funds_processed: u64,
    pub average_session_duration: i64,
    pub success_rate: f64,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub event_type: SecurityEventType,
    pub severity: Severity,
    pub description: String,
    pub ip_address: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    AnomalousSpending,
    UnauthorizedAccess,
    RateLimitExceeded,
    InvalidSignature,
    SessionHijacking,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub max_concurrent_sessions: usize,
    pub session_duration_secs: i64,
    pub inactivity_timeout_secs: i64,
    pub min_deposit_amount: u64,
    pub max_deposit_amount: u64,
    pub rate_limit_per_minute: usize,
    pub anomaly_detection_threshold: f64,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sessions: 1000,
            session_duration_secs: 3600, // 1 hour
            inactivity_timeout_secs: 1800, // 30 minutes
            min_deposit_amount: 5000, // 0.000005 SOL
            max_deposit_amount: 10_000_000_000, // 10 SOL
            rate_limit_per_minute: 100,
            anomaly_detection_threshold: 2.5,
        }
    }
}
