use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[derive(Serialize)]
struct ContactResponse {
    ok: bool,
    message: &'static str,
}

pub fn routes() -> Router {
    Router::new().route("/api/contact", post(handle_contact))
}

async fn handle_contact(
    Json(form): Json<ContactForm>,
) -> Result<Json<ContactResponse>, (StatusCode, Json<ContactResponse>)> {
    // Validate required fields
    let validation_err = |msg: &'static str| {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ContactResponse {
                ok: false,
                message: msg,
            }),
        ))
    };

    if form.name.trim().is_empty() {
        return validation_err("Name is required");
    }
    if !form.email.contains('@') || form.email.trim().len() < 5 {
        return validation_err("Valid email is required");
    }
    if form.subject.trim().is_empty() {
        return validation_err("Subject is required");
    }
    if form.message.trim().is_empty() {
        return validation_err("Message is required");
    }

    // Log the contact (in production, send email via smtp or write to db)
    info!(
        name = %form.name,
        email = %form.email,
        subject = %form.subject,
        "Contact form submission"
    );

    Ok(Json(ContactResponse {
        ok: true,
        message: "Thank you for your message. We will get back to you soon.",
    }))
}
