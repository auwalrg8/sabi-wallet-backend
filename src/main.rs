use axum::Router;
mod routes;
use sabi_wallet_backend::{config::Config, state::AppState};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "sabi_wallet_backend=debug,tower_http=info,breez_sdk=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = Config::from_env()?;
    let state = AppState::new(config).await?;

    // Run migrations on startup
    sqlx::migrate!("./migrations").run(&state.db).await?;

    let app = Router::new()
        .nest("/", routes::router())
        .with_state(state)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Sabi Wallet backend 🔛 Spark – listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
