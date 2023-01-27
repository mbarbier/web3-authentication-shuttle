use axum::{
    extract::{Path},
    routing::get,
    Router, Json,
};
use sync_wrapper::SyncWrapper;
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use tracing::{event, Level};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn nonce(query: Path<String>) -> Json<Value> {
    event!(Level::INFO, "Getting nonce for {}", query.0);
    let a = json!({ "data": { "nonce" : "abcd12345"} });
    Json(a)
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/nonce/:public_address", get(nonce))
        .layer(CorsLayer::permissive());
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
