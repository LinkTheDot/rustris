use crate::{game::game_config::GameConfig, general_data::logging};
use renderer::Renderer;

pub struct RustrisConfig {
  game: GameConfig,
  renderer: Renderer,
}

impl RustrisConfig {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn run(self) -> anyhow::Result<()> {
    Ok(())
  }
}

impl Default for RustrisConfig {
  fn default() -> Self {
    let _ = logging::setup_file_logger();
    let game = GameConfig::new();
    let renderer = Renderer::new();

    Self { game, renderer }
  }
}
