use axum::{
    extract::State,
    Json,
};
use uuid::Uuid;
use sabi_wallet_backend::{error::AppError, models::wallet::{CreateWalletRequest, CreateWalletResponse}, state::AppState};

pub async fn create_wallet(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> Result<Json<CreateWalletResponse>, AppError> {
    // 1. Device binding check
    let existing = sqlx::query("SELECT wallet_id FROM wallets WHERE device_id = ?")
        .bind(&payload.device_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if existing.is_some() {
        return Err(AppError::BadRequest("Device already bound to a wallet".into()));
    }

    // 2. Validate phone format
    if !payload.phone.starts_with("+234") || payload.phone.len() != 14 {
        return Err(AppError::BadRequest("Phone must be +234xxxxxxxxxx".into()));
    }

    // 3. Create node on Breez Spark (nodeless)
    let node = state.breez.create_node().await
        .map_err(|e| AppError::Internal(format!("Breez create_node failed: {e}")))?;

    // 4. Generate our internal wallet_id (UUID v7)
    let wallet_id = Uuid::now_v7().to_string();

    // 5. Save to DB
    sqlx::query(
        r#"
        INSERT INTO wallets (wallet_id, phone, device_id, node_pubkey, invite_code)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&wallet_id)
    .bind(&payload.phone)
    .bind(&payload.device_id)
    .bind(&node.node_pubkey)
    .bind(&node.invite_code)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(CreateWalletResponse {
        wallet_id,
        invite_code: node.invite_code,
        node_id: node.node_pubkey,
    }))
}