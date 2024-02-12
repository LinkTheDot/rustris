use crate::general_data::app_config;
use lazy_static::lazy_static;

lazy_static! {
  /// The configuration data for running the program.
  ///
  /// This will contain things such as log level, log message format, and other constant settings
  /// that may need to be set on compile time.
  // Will panic if the code for the creation of the config was incorrectly configured.
  pub static ref CONFIG: app_config::AppConfig = app_config::get_config().unwrap();
}

pub mod general_data {
  pub mod app_config;
  pub mod logging;
}

pub mod game {
  pub mod game_config;
  pub mod game_settings;
}

pub mod rustris_config;
