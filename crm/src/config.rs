use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
  pub port: u16,
  pub sender_email: String,
  pub metadata: String,
  pub user_stats: String,
  pub notification: String,
  pub tls: Option<TlsConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlsConfig {
  pub cert: String,
  pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
  pub pk: String,
}

impl AppConfig {
  pub fn load() -> Result<Self> {
    let ret = match (
      File::open("crm.yaml"),
      File::open("/etc/config/crm.yaml"),
      env::var("CRM_CONFIG"),
    ) {
      (Ok(reader), _, _) => serde_yaml::from_reader(reader),
      (_, Ok(reader), _) => serde_yaml::from_reader(reader),
      (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
      _ => bail!("No configuration file found"),
    };
    Ok(ret?)
  }
}
