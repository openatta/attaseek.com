use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use tracing::info;

mod config;
mod routes;

pub use config::AppConfig;

async fn spa_fallback(frontend_dir: Arc<PathBuf>) -> impl IntoResponse {
    let index_path = frontend_dir.join("index.html");
    match tokio::fs::read_to_string(&index_path).await {
        Ok(html) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(html))
            .unwrap(),
        Err(e) => {
            tracing::error!(?e, "Failed to read index.html");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(
                    "Frontend not built — run `npm run build` in frontend/",
                ))
                .unwrap()
        }
    }
}

pub fn build_app(config: &AppConfig) -> Router {
    let api = routes::api_routes(config);
    let frontend_dir = Arc::new(PathBuf::from(&config.frontend_dir));

    let serve_dir = ServeDir::new(frontend_dir.as_path())
        .precompressed_gzip()
        .precompressed_br();

    let fallback_dir = Arc::clone(&frontend_dir);

    Router::new()
        .merge(api)
        // Serve static files from frontend dist
        .fallback_service(serve_dir)
        // If file not found, serve index.html (SPA fallback)
        .fallback(move || {
            let dir = Arc::clone(&fallback_dir);
            async move { spa_fallback(dir).await }
        })
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
}

pub async fn serve(config: AppConfig) -> anyhow::Result<()> {
    let app = build_app(&config);
    let addr = config.listen;

    info!("AttaSeek web server starting on http://{addr}");
    info!("Serving frontend from: {}", config.frontend_dir);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    info!("Shutting down");
}
