use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Listen address (default 127.0.0.1:3001)
    #[serde(default = "default_listen")]
    pub listen: SocketAddr,

    /// Frontend dist directory to serve
    #[serde(default = "default_frontend_dir")]
    pub frontend_dir: String,

    /// Contact form recipient email
    #[serde(default)]
    pub contact_email: Option<String>,

    /// SMTP / sendgrid config (optional)
    #[serde(default)]
    pub smtp: Option<SmtpConfig>,

    /// Log level (default "info,attaseek_web=debug")
    #[serde(default = "default_log")]
    pub log: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

fn default_listen() -> SocketAddr {
    "127.0.0.1:3001".parse().unwrap()
}

fn default_frontend_dir() -> String {
    "../frontend/dist".into()
}

fn default_log() -> String {
    "info,attaseek_web=debug".into()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            listen: default_listen(),
            frontend_dir: default_frontend_dir(),
            contact_email: None,
            smtp: None,
            log: default_log(),
        }
    }
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let mut config: Self = Figment::new()
            .merge(Toml::file("attaseek.toml").nested())
            .merge(Env::prefixed("ATTASEEK_").split("__"))
            .extract()?;

        // Override port from PORT env var if set (simpler than figment mapping)
        if let Ok(port_str) = std::env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.listen.set_port(port);
            }
        }

        Ok(config)
    }
}
