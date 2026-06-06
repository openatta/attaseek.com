use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct NewsletterSub {
    pub email: String,
}

#[derive(Serialize)]
struct NewsletterResponse {
    ok: bool,
    message: &'static str,
}

pub fn routes() -> Router {
    Router::new().route("/api/newsletter", post(handle_newsletter))
}

async fn handle_newsletter(
    Json(sub): Json<NewsletterSub>,
) -> Result<Json<NewsletterResponse>, (StatusCode, Json<NewsletterResponse>)> {
    if !sub.email.contains('@') || sub.email.trim().len() < 5 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(NewsletterResponse {
                ok: false,
                message: "Valid email is required",
            }),
        ));
    }

    info!(email = %sub.email, "Newsletter subscription");

    Ok(Json(NewsletterResponse {
        ok: true,
        message: "Subscribed successfully.",
    }))
}
