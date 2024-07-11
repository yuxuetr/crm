use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
  pub port: u16,
  pub db_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
  pub pk: String,
}

impl AppConfig {
  pub fn load() -> Result<Self> {
    let ret = match (
      File::open("user_stat.yaml"),
      File::open("/etc/config/user_stat.yaml"),
      env::var("USER_STAT_CONFIG"),
    ) {
      (Ok(reader), _, _) => serde_yaml::from_reader(reader),
      (_, Ok(reader), _) => serde_yaml::from_reader(reader),
      (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
      _ => bail!("No configuration file found"),
    };
    Ok(ret?)
  }
}
