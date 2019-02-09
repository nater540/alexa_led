use config::{ConfigError, Config, File, Environment};
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Database {
  pub address: SocketAddr,
  pub username: Option<String>,
  pub password: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Listener {
  pub address: SocketAddr,
  pub backlog: Option<i16>,
  pub workers: Option<i16>
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub inbound_listener: Listener,
  pub database: Database
}

impl Settings {
  pub fn new(config_path: &str) -> Result<Self, ConfigError> {
    let mut cfg = Config::new();

    cfg.merge(File::with_name(config_path).required(false))?;
    cfg.merge(Environment::with_prefix("ragnarok").separator("_"))?;

    // Deserialize and freeze the entire configuration
    cfg.try_into()
  }
}
