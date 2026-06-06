use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config first so we can use its log filter
    let config = attaseek_web::AppConfig::load()?;

    // Setup tracing
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(false),
        )
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log)))
        .init();

    attaseek_web::serve(config).await
}
