use dotenv::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    pub vault_program_id: String,
    pub jwt_secret: String,
    pub max_concurrent_sessions: usize,
    pub session_duration_secs: i64,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/ephemeral_vault".to_string()),
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "http://localhost:8899".to_string()),
            solana_ws_url: env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "ws://localhost:8900".to_string()),
            vault_program_id: env::var("VAULT_PROGRAM_ID")
                .unwrap_or_else(|_| "11111111111111111111111111111111".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            max_concurrent_sessions: env::var("MAX_CONCURRENT_SESSIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            session_duration_secs: env::var("SESSION_DURATION_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
        }
    }
}
