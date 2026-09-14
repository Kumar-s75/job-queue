use std::{env, net::SocketAddr, time::Duration};

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub database_max_connections: u32,
    pub default_lease: Duration,
    pub recovery_interval: Duration,
    pub log_filter: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()).parse().expect("valid BIND_ADDR"),
            database_url: env::var("DATABASE_URL")?,
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(20),
            default_lease: Duration::from_secs(env::var("DEFAULT_LEASE_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(30)),
            recovery_interval: Duration::from_secs(env::var("RECOVERY_INTERVAL_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(5)),
            log_filter: env::var("RUST_LOG").unwrap_or_else(|_| "forgequeue=info,tower_http=info".into()),
        })
    }
}
