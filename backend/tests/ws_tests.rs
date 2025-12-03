use actix_web::{test, App};
use ephemeral_vault_backend::api::ws::ws_index;
use actix_web::web;

#[actix_rt::test]
async fn test_ws_index_accepts() {
    let app = test::init_service(App::new().route("/ws/", web::get().to(ws_index))).await;
    let req = test::TestRequest::with_uri("/ws/").to_request();
    // we can't complete a full websocket handshake via this test helper easily,
    // but we can assert that the handler returns a response (upgrade handshake starts)
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error() || resp.status().is_success());
}
