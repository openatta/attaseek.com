pub mod contact;
pub mod health;
pub mod newsletter;

use axum::Router;

use crate::config::AppConfig;

pub fn api_routes(_config: &AppConfig) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(contact::routes())
        .merge(newsletter::routes())
}
