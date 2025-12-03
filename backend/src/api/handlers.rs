use crate::{
    models::*,
    error::{AppError, AppResult},
    db::Database,
    managers::SessionManager,
    security::SecurityManager,
};
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub user_wallet: String,
    pub duration_secs: i64,
    pub device_info: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session_id: Uuid,
    pub ephemeral_wallet: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ApproveRequest {
    pub session_id: Uuid,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositRequest {
    pub session_id: Uuid,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RevokeRequest {
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct SessionStatusResponse {
    pub session_id: Uuid,
    pub is_active: bool,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub total_deposits: u64,
    pub total_spent: u64,
}

pub struct ApiHandlers {
    db: Database,
    session_manager: SessionManager,
    security: SecurityManager,
}

impl ApiHandlers {
    pub fn new(db: Database, session_manager: SessionManager) -> Self {
        Self {
            db,
            session_manager,
            security: SecurityManager::new(),
        }
    }

    /// POST /session/create - Create new ephemeral session
    pub async fn create_session(
        &self,
        req: HttpRequest,
        body: web::Json<CreateSessionRequest>,
    ) -> AppResult<HttpResponse> {
        // Get IP and user agent
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown");

        // Rate limiting
        self.security.check_rate_limit(&ip, 100).await?;

        // Validate wallet format
        if body.user_wallet.is_empty() {
            return Err(AppError::InvalidRequest("Invalid wallet".to_string()));
        }

        // Generate device fingerprint
        let device_fp = SecurityManager::generate_device_fingerprint(user_agent, &ip);

        // Create session
        let session = self
            .session_manager
            .create_session(
                &body.user_wallet,
                "vault_pda_placeholder", // Would be generated from on-chain
                &ip,
                Some(device_fp),
                body.duration_secs,
            )
            .await?;

        // Store in database
        self.db.create_session(&session).await?;

        Ok(HttpResponse::Ok().json(CreateSessionResponse {
            session_id: session.id,
            ephemeral_wallet: session.ephemeral_wallet,
            expires_at: session.expires_at,
        }))
    }

    /// POST /session/approve - Approve delegation
    pub async fn approve_delegation(
        &self,
        req: HttpRequest,
        body: web::Json<ApproveRequest>,
    ) -> AppResult<HttpResponse> {
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        // Get session
        let session = self.db.get_session(body.session_id).await?;

        // Verify session validity
        self.session_manager.verify_session(&session, &ip)?;

        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "approved",
            "session_id": body.session_id
        })))
    }

    /// DELETE /session/revoke - Revoke session and cleanup
    pub async fn revoke_session(
        &self,
        req: HttpRequest,
        body: web::Json<RevokeRequest>,
    ) -> AppResult<HttpResponse> {
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        let mut session = self.db.get_session(body.session_id).await?;
        self.session_manager.verify_session(&session, &ip)?;

        self.session_manager.revoke_session(&mut session);
        self.db.update_session(&session).await?;

        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "revoked",
            "session_id": body.session_id
        })))
    }

    /// GET /session/status - Get session info
    pub async fn get_session_status(
        &self,
        req: HttpRequest,
        session_id: web::Path<Uuid>,
    ) -> AppResult<HttpResponse> {
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        let session = self.db.get_session(*session_id).await?;
        self.session_manager.verify_session(&session, &ip)?;

        Ok(HttpResponse::Ok().json(SessionStatusResponse {
            session_id: session.id,
            is_active: session.is_active,
            expires_at: session.expires_at,
            total_deposits: session.total_deposits,
            total_spent: session.total_spent,
        }))
    }

    /// POST /session/deposit - Trigger auto-deposit
    pub async fn auto_deposit(
        &self,
        req: HttpRequest,
        body: web::Json<DepositRequest>,
    ) -> AppResult<HttpResponse> {
        let ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        let mut session = self.db.get_session(body.session_id).await?;
        self.session_manager.verify_session(&session, &ip)?;

        // Check for anomalies
        let is_anomalous = self
            .security
            .detect_anomaly(&session.user_wallet, body.amount, 2.5)
            .await?;

        if is_anomalous {
            return Err(AppError::SuspiciousActivity);
        }

        // Update session
        session.total_deposits = session.total_deposits.saturating_add(body.amount);
        self.db.update_session(&session).await?;

        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "deposited",
            "amount": body.amount,
            "new_balance": session.total_deposits
        })))
    }

    /// GET /analytics/user/:wallet - Get user analytics
    pub async fn get_analytics(
        &self,
        wallet: web::Path<String>,
    ) -> AppResult<HttpResponse> {
        let analytics = self.db.get_user_analytics(&wallet).await?;
        Ok(HttpResponse::Ok().json(analytics))
    }
}
