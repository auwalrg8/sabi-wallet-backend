use axum::{
    extract::State,
    Json,
};
use uuid::Uuid;
use sabi_wallet_backend::{
    error::AppError, 
    models::wallet::{CreateWalletRequest, CreateWalletResponse}, 
    state::AppState
};

pub async fn create_wallet(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> Result<Json<CreateWalletResponse>, AppError> {
    // Validate phone format
    if !payload.phone.starts_with("+234") || payload.phone.len() != 14 {
        return Err(AppError::BadRequest("Phone must be +234xxxxxxxxxx".into()));
    }

    tracing::info!(
        "User registered for social recovery → phone: {}, device: {}",
        payload.phone,
        payload.device_id
    );

    Ok(Json(CreateWalletResponse {
        invite_code: "sabi_2025_device_wallet".to_string(),
        node_id: "nodeless_spark_local".to_string(),
        initial_channel_opened: true,
    }))
}
