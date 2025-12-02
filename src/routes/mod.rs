pub mod wallets;
pub use wallets::create_wallet;

pub fn router() -> axum::Router<sabi_wallet_backend::state::AppState> {
    axum::Router::new()
        .route("/api/v1/wallets/create", axum::routing::post(create_wallet))
}