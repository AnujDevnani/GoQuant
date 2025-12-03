use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::*;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Self { pool })
    }

    // ==================== Sessions ====================

    pub async fn create_session(&self, session: &Session) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO sessions (
                id, user_wallet, ephemeral_wallet, ephemeral_keypair,
                vault_address, created_at, expires_at, is_active,
                total_deposits, total_spent, ip_address, device_fingerprint
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
        )
        .bind(session.id)
        .bind(&session.user_wallet)
        .bind(&session.ephemeral_wallet)
        .bind(&session.ephemeral_keypair)
        .bind(&session.vault_address)
        .bind(session.created_at)
        .bind(session.expires_at)
        .bind(session.is_active)
        .bind(session.total_deposits as i64)
        .bind(session.total_spent as i64)
        .bind(&session.ip_address)
        .bind(&session.device_fingerprint)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_session(&self, id: Uuid) -> AppResult<Session> {
        let row = sqlx::query(
            "SELECT id, user_wallet, ephemeral_wallet, ephemeral_keypair, vault_address, created_at, expires_at, is_active, total_deposits, total_spent, ip_address, device_fingerprint FROM sessions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::VaultNotFound)?;

        Ok(Session {
            id: row.get("id"),
            user_wallet: row.get("user_wallet"),
            ephemeral_wallet: row.get("ephemeral_wallet"),
            ephemeral_keypair: row.get("ephemeral_keypair"),
            vault_address: row.get("vault_address"),
            created_at: row.get("created_at"),
            expires_at: row.get("expires_at"),
            is_active: row.get("is_active"),
            total_deposits: row.get::<i64, _>("total_deposits") as u64,
            total_spent: row.get::<i64, _>("total_spent") as u64,
            ip_address: row.get("ip_address"),
            device_fingerprint: row.get("device_fingerprint"),
        })
    }

    pub async fn get_active_sessions(&self, user_wallet: &str) -> AppResult<Vec<Session>> {
        let rows = sqlx::query(
            "SELECT id, user_wallet, ephemeral_wallet, ephemeral_keypair, vault_address, created_at, expires_at, is_active, total_deposits, total_spent, ip_address, device_fingerprint FROM sessions WHERE user_wallet = $1 AND is_active = true AND expires_at > NOW()",
        )
        .bind(user_wallet)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|row| Session {
            id: row.get("id"),
            user_wallet: row.get("user_wallet"),
            ephemeral_wallet: row.get("ephemeral_wallet"),
            ephemeral_keypair: row.get("ephemeral_keypair"),
            vault_address: row.get("vault_address"),
            created_at: row.get("created_at"),
            expires_at: row.get("expires_at"),
            is_active: row.get("is_active"),
            total_deposits: row.get::<i64, _>("total_deposits") as u64,
            total_spent: row.get::<i64, _>("total_spent") as u64,
            ip_address: row.get("ip_address"),
            device_fingerprint: row.get("device_fingerprint"),
        }).collect())
    }

    pub async fn update_session(&self, session: &Session) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE sessions SET
                is_active = $1,
                total_deposits = $2,
                total_spent = $3
            WHERE id = $4
            "#,
        )
        .bind(session.is_active)
        .bind(session.total_deposits as i64)
        .bind(session.total_spent as i64)
        .bind(session.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // ==================== Transactions ====================

    pub async fn create_transaction(&self, tx: &VaultTransaction) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO vault_transactions (
                id, session_id, transaction_type, amount, fee, timestamp, status, signature
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
        )
        .bind(tx.id)
        .bind(tx.session_id)
        .bind(tx.transaction_type.to_string())
        .bind(tx.amount as i64)
        .bind(tx.fee as i64)
        .bind(tx.timestamp)
        .bind(tx.status.to_string())
        .bind(&tx.signature)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_session_transactions(&self, session_id: Uuid) -> AppResult<Vec<VaultTransaction>> {
        let rows = sqlx::query(
            "SELECT id, session_id, transaction_type, amount, fee, timestamp, status, signature FROM vault_transactions WHERE session_id = $1 ORDER BY timestamp DESC",
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|row| {
            let tx_type_str: String = row.get("transaction_type");
            let status_str: String = row.get("status");
            
            VaultTransaction {
                id: row.get("id"),
                session_id: row.get("session_id"),
                transaction_type: match tx_type_str.as_str() {
                    "deposit" => TransactionType::Deposit,
                    "trade" => TransactionType::Trade,
                    "withdrawal" => TransactionType::Withdrawal,
                    _ => TransactionType::Fee,
                },
                amount: row.get::<i64, _>("amount") as u64,
                fee: row.get::<i64, _>("fee") as u64,
                timestamp: row.get("timestamp"),
                status: match status_str.as_str() {
                    "pending" => TransactionStatus::Pending,
                    "confirmed" => TransactionStatus::Confirmed,
                    _ => TransactionStatus::Failed,
                },
                signature: row.get("signature"),
            }
        }).collect())
    }

    // ==================== Cleanup ====================

    pub async fn create_cleanup_event(&self, event: &CleanupEvent) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO cleanup_events (
                id, session_id, vault_address, returned_amount, cleaned_at, reason
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
            "#,
        )
        .bind(event.id)
        .bind(event.session_id)
        .bind(&event.vault_address)
        .bind(event.returned_amount as i64)
        .bind(event.cleaned_at)
        .bind(format!("{:?}", event.reason))
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // ==================== Analytics ====================

    pub async fn get_user_analytics(&self, user_wallet: &str) -> AppResult<SessionAnalytics> {
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(DISTINCT id) as total_sessions,
                COUNT(CASE WHEN is_active = true AND expires_at > NOW() THEN 1 END) as active_sessions,
                COALESCE(SUM(total_deposits), 0) as total_funds,
                EXTRACT(EPOCH FROM AVG(expires_at - created_at)) as avg_duration,
                MAX(CASE WHEN is_active = true THEN created_at ELSE NULL END) as last_activity
            FROM sessions
            WHERE user_wallet = $1
            "#,
        )
        .bind(user_wallet)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SessionAnalytics {
            user_wallet: user_wallet.to_string(),
            total_sessions: row.get("total_sessions"),
            active_sessions: row.get("active_sessions"),
            total_funds_processed: row.get::<i64, _>("total_funds") as u64,
            average_session_duration: row.get::<Option<f64>, _>("avg_duration")
                .unwrap_or(0.0) as i64,
            success_rate: 0.95, // Placeholder
            last_activity: row.get::<Option<DateTime<Utc>>, _>("last_activity")
                .unwrap_or_else(Utc::now),
        })
    }
}
