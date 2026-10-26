//! 12-factor configuration: environment variables -> typed struct (via `envy`).

use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    /// `DATABASE_URL`
    pub database_url: String,
    /// `JWT_SECRET` — required, there is deliberately no default.
    pub jwt_secret: String,
    /// `HOST`
    #[serde(default = "default_host")]
    pub host: String,
    /// `PORT`
    #[serde(default = "default_port")]
    pub port: u16,
    /// `LOG_JSON=true` switches logs to JSON (Loki, CloudWatch, Datadog).
    #[serde(default)]
    pub log_json: bool,
    /// `DB_MAX_CONNECTIONS`
    #[serde(default = "default_max_connections")]
    pub db_max_connections: u32,
    /// `REQUEST_TIMEOUT_SECS`
    #[serde(default = "default_timeout")]
    pub request_timeout_secs: u64,
    /// `CORS_ORIGINS` — comma-separated list; empty means permissive (dev only).
    #[serde(default)]
    pub cors_origins: String,
    /// `JWT_TTL_HOURS`
    #[serde(default = "default_jwt_ttl")]
    pub jwt_ttl_hours: i64,
}

fn default_host() -> String {
    "0.0.0.0".into()
}
fn default_port() -> u16 {
    3000
}
fn default_max_connections() -> u32 {
    20
}
fn default_timeout() -> u64 {
    10
}
fn default_jwt_ttl() -> i64 {
    12
}

impl Config {
    /// Reads `.env` (dev only — in production there is none) and the environment.
    /// A missing required variable stops the app at startup with a clear message.
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(envy::from_env::<Config>()?)
    }

    pub fn cors_origins(&self) -> Vec<String> {
        self.cors_origins
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
}

/// Hand-written `Debug`: secrets never end up in logs.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"***")
            .field("jwt_secret", &"***")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("log_json", &self.log_json)
            .field("db_max_connections", &self.db_max_connections)
            .field("request_timeout_secs", &self.request_timeout_secs)
            .field("cors_origins", &self.cors_origins)
            .field("jwt_ttl_hours", &self.jwt_ttl_hours)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_apply_and_debug_hides_secrets() {
        let vars = vec![
            (
                "DATABASE_URL".to_string(),
                "postgres://u:p@h/db".to_string(),
            ),
            ("JWT_SECRET".to_string(), "s3cr3t".to_string()),
        ];
        let c: Config = envy::from_iter(vars).unwrap();
        assert_eq!(c.port, 3000);
        assert_eq!(c.request_timeout_secs, 10);
        assert!(c.cors_origins().is_empty());
        let dbg = format!("{c:?}");
        assert!(!dbg.contains("s3cr3t") && !dbg.contains("u:p"));
    }

    #[test]
    fn missing_jwt_secret_is_an_error() {
        let vars = vec![("DATABASE_URL".to_string(), "postgres://x".to_string())];
        let err = envy::from_iter::<_, Config>(vars).unwrap_err();
        assert!(err.to_string().contains("jwt_secret"), "{err}");
    }
}
