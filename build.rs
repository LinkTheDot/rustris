extern crate git2;

use git2::*;
use std::env;

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> StdResult<()> {
  set_env_variable_if_doesnt_exist("LATEST_COMMIT_SHA", &get_commit_sha()?);

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
