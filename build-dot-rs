extern crate git2;

use git2::*;
use std::env;
use std::fs::DirBuilder;
use std::path::Path;

const PROJECT_DIR_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/");
const BUILD_DIR: &str = "builds/";

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

// | ./
// |- builds/
// |-- OS_ARCH/
// |--- profile/
// |---- rustris/
// |----- rustris       (the executable, configured to place in Rustris directory)
// |----- settings.toml (created on runtime)
// |----- data.bin      (created on runtime)

fn main() -> StdResult<()> {
  let build_architecture = env::var("CARGO_CFG_TARGET_ARCH")?;
  let build_operation_system = env::var("CARGO_CFG_TARGET_OS")?;
  let build_level = env::var("PROFILE")?;
  let build_path = format!(
    "{}{}_{}/{}/",
    BUILD_DIR, build_operation_system, build_architecture, build_level
  );

  match build_level.to_lowercase().trim() {
    "debug" => {
      set_env_variable_if_doesnt_exist("LOG_LEVEL", "debug");
      set_env_variable_if_doesnt_exist("LOG_FORMAT", "longest");
    }

    _ => {
      set_env_variable_if_doesnt_exist("LOG_LEVEL", "info");
      set_env_variable_if_doesnt_exist("LOG_FORMAT", "short");
    }
  };

  if build_operation_system.contains("windows") {
    handle_file_creation(&build_path)?;
  }

  let latest_commit_sha = get_commit_sha()?;

  println!("cargo:rustc-env=LATEST_COMMIT_SHA={}", latest_commit_sha);

  Ok(())
}

/// Builds out the tree of directories for the program.
fn handle_file_creation<P: AsRef<Path>>(build_path: P) -> StdResult<()> {
  let rustris_dir_path = build_path.as_ref().join(PROJECT_DIR_NAME);

  DirBuilder::new().recursive(true).create(rustris_dir_path)?;

  Ok(())
}

fn get_commit_sha() -> StdResult<String> {
  let repo_path = ".";

  let repo = Repository::open(repo_path)?;
  let head = repo.head()?;
  let latest_commit = head.peel_to_commit()?;

  println!("Latest commit SHA: {}", latest_commit.id());

  Ok(latest_commit.id().to_string())
}

fn set_env_variable_if_doesnt_exist(environment_variable: &str, value: &str) {
  if env::var(environment_variable).is_err() {
    println!("cargo:rustc-env={}={}", environment_variable, value);
  }
}
