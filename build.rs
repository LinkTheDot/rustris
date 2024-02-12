use std::env;
use std::fs::DirBuilder;
use std::path::Path;

const PROJECT_DIR_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/");
const BUILD_DIR: &str = "builds/";

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

// | config.toml (stored in the binary)
// |- rustris/       (created if it doesn't exist)
// |-- rustris       (the executable, configured to place in Rustris directory)
// |-- settings.toml (created on runtime)
// |-- data.bin      (created on runtime)

fn main() -> StdResult<()> {
  let build_architecture = env::var("CARGO_CFG_TARGET_ARCH")?;
  let build_operation_system = env::var("CARGO_CFG_TARGET_OS")?;
  let build_level = env::var("PROFILE")?;
  let build_path = format!(
    "{}{}_{}/{}/",
    BUILD_DIR, build_operation_system, build_architecture, build_level
  );

  handle_file_creation(&build_path)?;

  Ok(())
}

/// Builds out the tree of directories for the program.
fn handle_file_creation<P: AsRef<Path>>(build_path: P) -> StdResult<()> {
  let rustris_dir_path = build_path.as_ref().join(PROJECT_DIR_NAME);

  DirBuilder::new().recursive(true).create(rustris_dir_path)?;

  Ok(())
}

// Settings file creation for later reference.
// This should live in the code since it needs opening and checked at runtime anyways.
//
// let settings_file_path = Path::new("settings.toml");
// if !settings_file_path.exists() {
//   create_settings_file(settings_file_path)?;
// }
// OpenOptions::new()
//   .create_new(true)
//   .open(settings_file_path.as_ref())?;
