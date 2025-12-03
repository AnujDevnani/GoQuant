use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use ephemeral_vault_backend::{
    config::Config,
    db::Database,
    managers::SessionManager,
    api::handlers::ApiHandlers,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();
    
    log::info!("Starting Ephemeral Vault Backend");
    log::info!("Database: {}", config.database_url);
    log::info!("Solana RPC: {}", config.solana_rpc_url);

    // Initialize database
    let db = Database::new(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize managers
    let session_manager = SessionManager::new(&config.jwt_secret)
        .expect("Failed to initialize session manager");

    let handlers = Arc::new(ApiHandlers::new(db, session_manager));

    let port = config.port;
    let server_addr = format!("127.0.0.1:{}", port);

    log::info!("Starting server on {}", server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlers.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/session/create", web::post().to(
                        |req: actix_web::HttpRequest, body: web::Json<ephemeral_vault_backend::api::handlers::CreateSessionRequest>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.create_session(req, body).await
                        }
                    ))
                    .route("/session/approve", web::post().to(
                        |req: actix_web::HttpRequest, body: web::Json<ephemeral_vault_backend::api::handlers::ApproveRequest>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.approve_delegation(req, body).await
                        }
                    ))
                    .route("/session/revoke", web::delete().to(
                        |req: actix_web::HttpRequest, body: web::Json<ephemeral_vault_backend::api::handlers::RevokeRequest>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.revoke_session(req, body).await
                        }
                    ))
                    .route("/session/{session_id}/status", web::get().to(
                        |req: actix_web::HttpRequest, session_id: web::Path<uuid::Uuid>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.get_session_status(req, session_id).await
                        }
                    ))
                    .route("/session/deposit", web::post().to(
                        |req: actix_web::HttpRequest, body: web::Json<ephemeral_vault_backend::api::handlers::DepositRequest>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.auto_deposit(req, body).await
                        }
                    ))
                    .route("/analytics/user/{wallet}", web::get().to(
                        |req: actix_web::HttpRequest, wallet: web::Path<String>| async move {
                            let handlers_data = req.app_data::<web::Data<Arc<ApiHandlers>>>()
                                .expect("Failed to get handlers").get_ref().clone();
                            handlers_data.get_analytics(wallet).await
                        }
                    ))
                    .service(web::resource("/ws/").route(web::get().to(ephemeral_vault_backend::api::ws::ws_index)))
            )
    })
    .bind(&server_addr)?
    .run()
    .await
}
