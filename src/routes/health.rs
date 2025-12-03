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
            message: None,
            latency_ms: Some(db_start.elapsed().as_millis() as u64),
        },
        Err(e) => {
            overall_status = "degraded";
            ServiceHealth {
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
                latency_ms: None,
            }
        }
    };

    // 2. Check Breez service connectivity
    let breez_start = Instant::now();
    let breez_health = match state.breez.health_check().await {
        Ok(_) => ServiceHealth {
            status: "healthy".to_string(),
            message: None,
            latency_ms: Some(breez_start.elapsed().as_millis() as u64),
        },
        Err(e) => {
            overall_status = "degraded";
            ServiceHealth {
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
                latency_ms: None,
            }
        }
    };

    // 3. Check LSP connectivity (via Breez service)
    let lsp_start = Instant::now();
    let lsp_health = match state.breez.check_lsp_status().await {
        Ok(status) => ServiceHealth {
            status: if status.is_online { "healthy" } else { "degraded" }.to_string(),
            message: Some(format!("LSP: {}", status.lsp_id)),
            latency_ms: Some(lsp_start.elapsed().as_millis() as u64),
        },
        Err(e) => {
            overall_status = "degraded";
            ServiceHealth {
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
                latency_ms: None,
            }
        }
    };

    Ok(Json(HealthCheckResponse {
        status: overall_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: db_health,
        breez_service: breez_health,
        lsp: lsp_health,
    }))
}
