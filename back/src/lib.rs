use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str;
use sync_wrapper::SyncWrapper;
use tower_http::cors::CorsLayer;
use tracing::{event, Level};

mod web3;

#[derive(Deserialize)]
struct AuthenticateParams {
    address: String,
    nonce: String,
    signature: String,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn nonce(query: Path<String>) -> Json<Value> {
    event!(Level::INFO, "Getting nonce for {}", query.0);
    let nonce = "app_abcd12345";
    Json(json!({ "data": { "nonce" : nonce} }))
}

async fn authenticate(Query(params): Query<AuthenticateParams>) -> Json<Value> {
    event!(
        Level::INFO,
        "Authenticating {}, {}, {}",
        params.address,
        params.nonce,
        params.signature
    );

    let sign_message = web3::get_sign_message(params.nonce);
    let signature = hex::decode(&params.signature).unwrap();
    let address = web3::recover(&sign_message, &signature);

    let success = if address == params.address {
        true
    } else {
        false
    };

    event!(Level::INFO, "Address {}", &address);

    let a = json!({ "data": { "success" : success} });
    Json(a)
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/nonce/:public_address", get(nonce))
        .route("/authenticate", get(authenticate))
        .layer(CorsLayer::permissive());
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
