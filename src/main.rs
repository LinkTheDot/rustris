#![allow(clippy::needless_return)]

use rustris::rustris_config::RustrisConfig;

fn main() {
  let _ = rustris::general_data::logging::setup_file_logger();

  if let Err(error) = std::panic::catch_unwind(run_game) {
    let error = if let Some(error) = error.downcast_ref::<&'static str>() {
      error
    } else if let Some(error) = error.downcast_ref::<String>() {
      error.as_str()
    } else {
      "Unknown reason"
    };

    log::error!(
      "A fatal error occurred when running the game: `{:?}`",
      error
    );

    std::process::exit(1);
  } else {
    std::process::exit(0);
  }
}

#[inline]
fn run_game() {
  let (config, event_loop, window) = match RustrisConfig::new() {
    Ok(values) => values,
    Err(error) => {
      log::error!("Failed to load the config: `{:?}`", error);

      return;
    }
  };

  if let Err(error) = config.run(event_loop, window) {
    log::error!(
      "An unrecoverable logical error occurred when running the game: `{:?}`",
      error
    );

    return;
  }
}
