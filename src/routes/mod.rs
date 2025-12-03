pub mod wallets;
pub mod health;

pub use wallets::create_wallet;
pub use health::health_check;

pub fn router() -> axum::Router<sabi_wallet_backend::state::AppState> {
    axum::Router::new()
        .route("/api/v1/wallets/create", axum::routing::post(create_wallet))
        .route("/health", axum::routing::get(health_check))
}