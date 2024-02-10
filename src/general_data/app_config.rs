#![cfg(not(tarpaulin_include))]

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

// The config crate doesn't have a way I found to use an array of bytes.
// The alternative is just implementing the entire crate manually, or importing it and changing things
// in their crate.
// const CONFIG_FILE_DATA: &[u8] = include_bytes!(concat!(env!("PWD"), "/config.toml"));

/// The possible log levels are trace, info, error, warn, and debug.
const LOG_LEVEL: Option<&str> = None;
/// The possible log message sizes are long, short, and shortest.
const LOG_FILE_MESSAGE_SIZE: Option<&str> = None;

/// The list of options for the application.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
  pub log_level: String,
  pub log_file_message_size: String,
}

impl AppConfig {
  const DEFAULT_LOG_LEVEL_NAME: &str = "log_level";
  const DEFAULT_LOG_LEVEL_VALUE: &str = "debug";

  const DEFAULT_LOG_MESSAGE_SIZE_NAME: &str = "log_file_message_size";
  const DEFAULT_LOG_MESSAGE_SIZE_VALUE: &str = "long";
}

impl Default for AppConfig {
  fn default() -> Self {
    Self {
      log_level: AppConfig::DEFAULT_LOG_LEVEL_VALUE.to_string(),
      log_file_message_size: AppConfig::DEFAULT_LOG_MESSAGE_SIZE_VALUE.to_string(),
    }
  }
}

pub fn get_config() -> Result<AppConfig, ConfigError> {
  let default_config_data = AppConfig::default();

  let mut config_builder = Config::builder();

  // Set overrides
  config_builder = config_builder
    .set_override_option(AppConfig::DEFAULT_LOG_LEVEL_NAME, LOG_LEVEL)?
    .set_override_option(
      AppConfig::DEFAULT_LOG_MESSAGE_SIZE_NAME,
      LOG_FILE_MESSAGE_SIZE,
    )?;

  // Set defaults
  config_builder = config_builder
    .set_default(
      AppConfig::DEFAULT_LOG_LEVEL_NAME,
      default_config_data.log_level,
    )?
    .set_default(
      AppConfig::DEFAULT_LOG_MESSAGE_SIZE_NAME,
      default_config_data.log_file_message_size,
    )?;

  // Build
  config_builder.build()?.try_deserialize()
}
