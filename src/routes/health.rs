use axum::{extract::State, Json};
use sabi_wallet_backend::{error::AppError, models::wallet::{HealthCheckResponse, ServiceHealth}, state::AppState};
use std::time::Instant;

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthCheckResponse>, AppError> {
    let mut overall_status = "healthy";

    // 1. Check database connectivity
    let db_start = Instant::now();
    let db_health = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => ServiceHealth {
            status: "healthy".to_string(),
            message: Some("Database connected".to_string()),
            latency_ms: Some(db_start.elapsed().as_millis() as u64),
        },
        Err(e) => {
            overall_status = "unhealthy";
            ServiceHealth {
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
                latency_ms: None,
            }
        }
    };

    // 2. Breez SDK Nodeless (2025) - No backend service needed!
    // All Lightning operations happen on the device via Breez SDK
    // No LSP check needed from backend - client SDK handles this
    let breez_health = ServiceHealth {
        status: "n/a".to_string(),
        message: Some("Breez SDK runs on device - no backend service".to_string()),
        latency_ms: None,
    };

    let lsp_health = ServiceHealth {
        status: "n/a".to_string(),
        message: Some("LSP checked by device SDK - not backend".to_string()),
        latency_ms: None,
    };

    Ok(Json(HealthCheckResponse {
        status: overall_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: db_health,
        breez_service: breez_health,
        lsp: lsp_health,
    }))
}
