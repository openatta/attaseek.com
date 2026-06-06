use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: &'static str,
    version: &'static str,
}

pub fn routes() -> Router {
    Router::new().route(
        "/health",
        get(|| async {
            Json(Health {
                status: "ok",
                version: env!("CARGO_PKG_VERSION"),
            })
        }),
    )
}
