pub mod wallets;
pub mod health;

pub use wallets::{create_wallet, get_wallet_status};
pub use health::health_check;

pub fn router() -> axum::Router<sabi_wallet_backend::state::AppState> {
    axum::Router::new()
        .route("/api/v1/wallets/create", axum::routing::post(create_wallet))
        .route("/api/v1/wallets/:wallet_id/:device_id/status", axum::routing::get(get_wallet_status))
        .route("/health", axum::routing::get(health_check))
}