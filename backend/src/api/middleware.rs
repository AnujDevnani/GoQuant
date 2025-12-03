use actix_web::{web, HttpResponse};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // These route registrations are placeholders to keep the module compiling.
    // The actual handlers are registered in `main.rs` with access to
    // `web::Data<Arc<ApiHandlers>>`. Keeping thin placeholders here avoids
    // requiring `ApiHandlers` to implement `FromRequest`.
    cfg.service(
        web::scope("/session")
            .route("/create", web::post().to(|| async { HttpResponse::NotImplemented() }))
            .route("/approve", web::post().to(|| async { HttpResponse::NotImplemented() }))
            .route("/revoke", web::delete().to(|| async { HttpResponse::NotImplemented() }))
            .route("/{session_id}/status", web::get().to(|| async { HttpResponse::NotImplemented() }))
            .route("/deposit", web::post().to(|| async { HttpResponse::NotImplemented() }))
    )
    .service(
        web::scope("/analytics").route("/user/{wallet}", web::get().to(|| async { HttpResponse::NotImplemented() })),
    );
}
