use axum::{
    extract::{State, Path},
    Json,
};
use uuid::Uuid;
use sabi_wallet_backend::{
    error::AppError, 
    models::wallet::{CreateWalletRequest, CreateWalletResponse, WalletStatusResponse, ChannelStatus}, 
    state::AppState
};

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

    // 4. Generate recovery phrase if backup_type is "seed" (Classic 12-word)
    // NOTE: In production, this should be generated CLIENT-SIDE by Breez SDK
    // We only return it here if client explicitly requests server-generated seed
    let recovery_phrase = if backup_type == "seed" {
        use bip39::Mnemonic;
        let mnemonic = Mnemonic::generate(12)
            .map_err(|e| AppError::Internal(format!("Failed to generate mnemonic: {e}")))?;
        Some(mnemonic.to_string())
    } else {
        None
    };

    // 5. Generate wallet_id (UUID v7) - our internal identifier
    let wallet_id = Uuid::now_v7().to_string();

    // 6. IMPORTANT: Breez SDK Nodeless (2025) - wallet creation happens ON DEVICE
    // The SDK on the client device:
    //   - Generates the mnemonic locally
    //   - Connects to Breez using API key
    //   - Opens channel instantly via LSP + Liquid swaps
    //   - Never sends private keys to any server
    // Backend only stores metadata for P2P trades, social recovery, USSD
    
    let invite_code = format!("sabi_{}", &wallet_id[..8]);
    let node_id = "nodeless-device-managed"; // Real node_id comes from client SDK
    
    // 7. Channel is opened instantly by Breez LSP on device (100% non-custodial)
    let first_channel_opened: i64 = 1; // Breez opens inbound liquidity instantly
    let first_channel_sats: i64 = state
        .config
        .first_channel_sats_default
        .unwrap_or(200_000)
        .clamp(100_000, 300_000);

    // 8. Save metadata to backend DB
    let now = chrono::Utc::now();
    let recovery_phrase_shown = if recovery_phrase.is_some() { 1 } else { 0 };
    
    sqlx::query(
        r#"
        INSERT INTO wallets (
            wallet_id, phone, device_id, breez_node_id, invite_code, 
            backup_type, backup_status, first_channel_opened, first_channel_sats,
            device_bound_at, recovery_phrase_shown, last_seen_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&wallet_id)
    .bind(&payload.phone)
    .bind(&payload.device_id)
    .bind(node_id)
    .bind(&invite_code)
    .bind(&backup_type)
    .bind(backup_status)
    .bind(first_channel_opened)
    .bind(first_channel_sats)
    .bind(now)
    .bind(recovery_phrase_shown)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::info!(
        wallet_id,
        phone = %payload.phone,
        backup_type,
        "✅ Wallet metadata stored (Lightning wallet created on device by Breez SDK)"
    );

    Ok(Json(CreateWalletResponse {
        invite_code,
        node_id: node_id.to_string(),
        initial_channel_opened: true, // Breez opens instantly via LSP
        recovery_phrase,
    }))
}

pub async fn get_wallet_status(
    State(state): State<AppState>,
    Path((wallet_id, device_id)): Path<(String, String)>,
) -> Result<Json<WalletStatusResponse>, AppError> {
    // 1. Fetch wallet and verify device binding
    let wallet: Option<sabi_wallet_backend::models::wallet::Wallet> = sqlx::query_as(
        "SELECT * FROM wallets WHERE wallet_id = ?"
    )
    .bind(&wallet_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let wallet = wallet.ok_or_else(|| AppError::BadRequest("Wallet not found".into()))?;

    // 2. Enforce device binding - prevent access from different device
    if wallet.device_id != device_id {
        tracing::warn!(
            wallet_id, 
            bound_device = %wallet.device_id, 
            attempted_device = %device_id,
            "⚠️ Device binding violation detected"
        );
        return Err(AppError::BadRequest("This wallet is bound to another device".into()));
    }

    // 3. Update last_seen_at
    let now = chrono::Utc::now();
    sqlx::query("UPDATE wallets SET last_seen_at = ? WHERE wallet_id = ?")
        .bind(now)
        .bind(&wallet_id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 4. Return wallet metadata
    // NOTE: Real-time balance and channel status come from Breez SDK on the device
    // Backend only stores metadata - client should call SDK.nodeInfo() for live data
    
    Ok(Json(WalletStatusResponse {
        wallet_id: wallet.wallet_id,
        status: wallet.status,
        balance_sats: 0, // Real balance from device SDK - client updates this
        channel_status: ChannelStatus {
            has_channel: wallet.first_channel_opened == 1,
            channel_capacity_sats: wallet.first_channel_sats,
            is_connected: false, // Connection status from device SDK
        },
        device_id: wallet.device_id,
        last_seen: wallet.last_seen_at.map(|dt| dt.to_rfc3339()),
        backup_status: wallet.backup_status,
    }))
}
