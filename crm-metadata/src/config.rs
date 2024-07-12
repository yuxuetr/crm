use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
  pub auth: AuthConfig,
  pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
  pub pk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
  pub port: u16,
}

impl AppConfig {
  pub fn load() -> Result<Self> {
    let ret = match (
      File::open("metadata.yaml"),
      File::open("/etc/config/metadata.yaml"),
      env::var("METADATA_CONFIG"),
    ) {
      (Ok(f), _, _) => serde_yaml::from_reader(f),
      (_, Ok(f), _) => serde_yaml::from_reader(f),
      (_, _, Ok(f)) => serde_yaml::from_str(&f),
      _ => bail!("No configuration file found"),
    };
    Ok(ret?)
  }
}
