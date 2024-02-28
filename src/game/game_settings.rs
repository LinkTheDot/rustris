// use std::collections::HashMap;

// This will contain things like controls, ui scaling, textures, and more.
pub struct GameSettings {
  /// The current set fps.
  fps: u32,
  controls: Controls,
}

struct Controls {
  // inner: HashMap<String,
}

impl GameSettings {
  pub fn initialize() -> anyhow::Result<Self> {
    let controls = Controls::initialize()?;

    Ok(Self { fps: 144, controls })
  }

  /// The current set fps.
  ///
  /// Clamped to 20, 144.
  pub fn fps(&self) -> u32 {
    self.fps.clamp(20, 144)
  }
}

impl Controls {
  fn initialize() -> anyhow::Result<Self> {
    Ok(Self {})
  }
}
