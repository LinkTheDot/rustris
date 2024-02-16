use chrono::Utc;
use log::LevelFilter;
use log4rs::{
  append::file::FileAppender,
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
};
use std::str::FromStr;

const LATEST_COMMIT_SHA: &str = env!("LATEST_COMMIT_SHA");

/// Creates a new log file in "crate/logs/".
/// The new log file will be named after the current time and date based on UTC.
/// The name format is as such Y-M-D-H:M:S-UTC or Year-Month-Day-Hour:Minute:Second-TimeZone.
///
/// The format in the log file can be multiple options.
/// The default is "Long".
///
/// long: "(Hour:Minute:Second)(TimeZone) | FilePath: Line | Level - Message".
/// short: "FilePath: Line | Level - Message".
/// shortest: Level - "Message"
#[cfg(not(tarpaulin_include))]
pub fn setup_file_logger() -> Result<log4rs::Handle, Box<dyn std::error::Error>> {
  let date = Utc::now().to_string().replace(':', " ");
  let log_file_path = format!("logs/{date}").replace(' ', "-");
  let logging_format = get_logging_format();
  let log_level = LevelFilter::from_str(env!("LOG_LEVEL")).unwrap_or(LevelFilter::Info);

  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(&logging_format)))
    .build(log_file_path)?;

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder().appender("logfile").build(log_level))?;

  log::warn!(
    "This build is: {}\n",
    LATEST_COMMIT_SHA.chars().take(10).collect::<String>()
  );

  log4rs::init_config(config).map_err(Into::into)
}

/// To get the list of possible fields refer to the docs listed below:
/// https://docs.rs/log4rs/1.2.0/log4rs/encode/pattern/index.html#formatters
fn get_logging_format() -> String {
  let logging_format = match env!("LOG_FORMAT").to_lowercase().trim() {
    "long" => "{d(%H:%M:%S %Z)(utc)} | {f}: {L} | {l} - {m}\n",
    "short" => "{d(%H:%M:%S %Z)(utc)} | {l} - {m}\n",
    "shortest" => "{m}\n",
    _ => "{d(%H:%M:%S %Z)(utc)} | {l} - {m}\n",
  };

  logging_format.to_string()
}
