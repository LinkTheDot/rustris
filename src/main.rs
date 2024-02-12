use rustris::rustris_config::RustrisConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = RustrisConfig::new()?;

  config.run().map_err(Into::into)
}
