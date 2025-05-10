use axum::extract::Query;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;

#[derive(Deserialize)]
pub struct WalletQuery {
    wallet_address: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    receive_wallet: String,
    processed_by: String,
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root(Query(params): Query<WalletQuery>) -> Json<ApiResponse> {
    // format!("Hello Axum! Wallet Address: {}", params.wallet_address)
    let response_data = ApiResponse {
        message: "Hello from Axum on Cloudflare Workers!".to_string(),
        receive_wallet: params.wallet_address,
        processed_by: "Rust Workers".to_string(),
    };
    Json(response_data)
}