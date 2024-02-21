use rustris::rustris_config::RustrisConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (config, event_loop) = RustrisConfig::new()?;

  config.run(event_loop).map_err(Into::into)
}
