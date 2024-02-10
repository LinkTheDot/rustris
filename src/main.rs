use rustris::rustris_config::RustrisConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  RustrisConfig::new().run().map_err(Into::into)
}
