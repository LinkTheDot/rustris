// use std::collections::HashMap;

// This will contain things like controls, ui scaling, textures, and more.
pub struct GameSettings {
  controls: Controls,
}

struct Controls {
  // inner: HashMap<String,
}

impl GameSettings {
  pub fn initialize() -> anyhow::Result<Self> {
    let controls = Controls::initialize()?;

    Ok(Self { controls })
  }
}

impl Controls {
  fn initialize() -> anyhow::Result<Self> {
    Ok(Self {})
  }
}
