use axum::{
    extract::{Path, Query},
    routing::get,
    Extension, Json, Router,
};
use errors::AppError;
use rand::{distributions::Alphanumeric, Rng};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;
use std::str;
use sync_wrapper::SyncWrapper;
use tower_http::cors::CorsLayer;
use tracing::{event, Level};

mod errors;
mod web3;

async fn nonce(
    address: Path<String>,
    Extension(database): Extension<sqlx::PgPool>,
) -> Result<Json<Value>, AppError> {
    event!(Level::INFO, "Getting nonce for {}", address.0);

    let rand = ChaChaRng::from_entropy();
    let val: String = rand
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let val = "app_".to_string() + &val;

    sqlx::query(
        "
    INSERT INTO users (address, nonce, authentication) 
    VALUES ($1, $2, $3)
    ON CONFLICT (address) DO UPDATE 
        SET nonce = excluded.nonce;
    ",
    )
    .bind(&address.0)
    .bind(&val)
    .bind(0)
    .execute(&database)
    .await?;

    Ok(Json(json!({ "data": { "nonce" : val} })))
}

#[derive(Deserialize)]
struct AuthenticateParams {
    address: String,
    nonce: String,
    signature: String,
}
async fn authenticate(
    Query(params): Query<AuthenticateParams>,
    Extension(database): Extension<sqlx::PgPool>,
) -> Result<Json<Value>, AppError> {
    event!(
        Level::INFO,
        "Authenticating {}, {}, {}",
        params.address,
        params.nonce,
        params.signature
    );

    let row = sqlx::query("SELECT id, nonce, authentication FROM users WHERE address = $1;")
        .bind(&params.address)
        .fetch_one(&database)
        .await?;
    let user_id: i32 = row.get("id");
    let visit: i32 = row.get("authentication");
    let nonce: String = row.get("nonce");
    if nonce != params.nonce {
        return Err(AppError::Generic("Invalid nonce".to_string()));
    }

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

    sqlx::query("UPDATE users SET nonce = '', authentication = $2 WHERE id = $1;")
        .bind(&user_id)
        .bind(visit + 1)
        .execute(&database)
        .await?;

    let a = json!({ "data": { "success" : success} });
    Ok(Json(a))
}

#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/nonce/:public_address", get(nonce))
        .route("/authenticate", get(authenticate))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive());

    let sync_wrapper = SyncWrapper::new(router);
    Ok(sync_wrapper)
}
