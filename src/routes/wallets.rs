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

    // 3. Validate backup_type
    let backup_type = payload.backup_type.to_lowercase();
    if !matches!(backup_type.as_str(), "none" | "social" | "seed") {
        return Err(AppError::BadRequest("backup_type must be 'none', 'social', or 'seed'".into()));
    }

    let backup_status = if backup_type == "none" { "skipped" } else { "pending" };

    // 3. Create node on Breez Spark (nodeless)
    let node = state.breez.create_node().await
        .map_err(|e| AppError::Internal(format!("Breez create_node failed: {e}")))?;

    // 4. Generate our internal wallet_id (UUID v7)
    let wallet_id = Uuid::now_v7().to_string();

    // 5. Decide first channel liquidity (100k–300k sats)
    let default_sats = state
        .config
        .first_channel_sats_default
        .unwrap_or(200_000);
    let first_channel_sats = default_sats.clamp(100_000, 300_000);

    // 6. Attempt to open first channel via Breez
    let mut first_channel_opened: i64 = 0;
    if let Err(e) = state.breez.open_first_channel(&wallet_id, first_channel_sats).await {
        // Do not fail wallet creation; just record not opened yet
        tracing::warn!(error = %e, "Failed to open first channel");
    } else {
        first_channel_opened = 1;
    }

    // 7. Save to DB
    sqlx::query(
        r#"
        INSERT INTO wallets (wallet_id, phone, device_id, breez_node_id, invite_code, backup_type, backup_status, first_channel_opened, first_channel_sats)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&wallet_id)
    .bind(&payload.phone)
    .bind(&payload.device_id)
    .bind(&node.node_pubkey)
    .bind(&node.invite_code)
    .bind(&backup_type)
    .bind(backup_status)
    .bind(first_channel_opened)
    .bind(first_channel_sats)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(CreateWalletResponse {
        invite_code: node.invite_code,
        node_id: node.node_pubkey,
        initial_channel_opened: first_channel_opened == 1,
    }))
}