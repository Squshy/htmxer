pub mod config;
pub mod handlers;
pub mod templates;

use axum::{routing::get, Router};
use config::AppState;
use handlers::home;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing app...");

    let assets_path = std::env::current_dir().unwrap();
    let app_state = AppState::new();

    let app = Router::new()
        .route("/", get(home))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state);

    let port = 42069;
    info!("app initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
