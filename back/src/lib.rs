use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use errors::AppError;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str;
use sync_wrapper::SyncWrapper;
use tower_http::cors::CorsLayer;
use tracing::{event, Level};

mod errors;
mod web3;

async fn nonce(query: Path<String>) -> Result<Json<Value>, AppError> {
    event!(Level::INFO, "Getting nonce for {}", query.0);
    let nonce = "app_abcd12345";
    Ok(Json(json!({ "data": { "nonce" : nonce} })))
}

#[derive(Deserialize)]
struct AuthenticateParams {
    address: String,
    nonce: String,
    signature: String,
}
async fn authenticate(Query(params): Query<AuthenticateParams>) -> Result<Json<Value>, AppError> {
    event!(
        Level::INFO,
        "Authenticating {}, {}, {}",
        params.address,
        params.nonce,
        params.signature
    );

    let sign_message = web3::get_sign_message(params.nonce);
    let signature = hex::decode(&params.signature)
        .or_else(|_| Err(AppError::Generic("Invalid signature".to_string())))?;
    let address = web3::recover(&sign_message, &signature)
        .or_else(|e| Err(AppError::Generic(e.to_string())))?;

    let success = if address == params.address {
        true
    } else {
        false
    };

    let a = json!({ "data": { "success" : success} });
    Ok(Json(a))
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/nonce/:public_address", get(nonce))
        .route("/authenticate", get(authenticate))
        .layer(CorsLayer::permissive());
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
