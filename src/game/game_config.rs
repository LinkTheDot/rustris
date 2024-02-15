use winit_input_helper::WinitInputHelper;

pub struct GameConfig {}

impl GameConfig {
  pub fn new() -> Self {
    Self {}
  }

  pub fn handle_input(&mut self, input: &WinitInputHelper) -> anyhow::Result<()> {
    Ok(())
  }
}
