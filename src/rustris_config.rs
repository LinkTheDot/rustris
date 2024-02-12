use crate::game::game_config::GameConfig;
use crate::game::game_settings::GameSettings;
use crate::general_data::logging;
use renderer::Renderer;
use winit::dpi::*;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct RustrisConfig {
  game: GameConfig,
  renderer: Renderer,
  window: Window,
  settings: GameSettings,
}

impl RustrisConfig {
  pub fn new() -> anyhow::Result<Self> {
    // The handle isn't required for current uses
    let _ = logging::setup_file_logger();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
      .with_title("Rustris")
      .with_inner_size(LogicalSize::new(500_u32, 900_u32))
      .with_resizable(false)
      // .with_maximized(true)
      .build(&event_loop)?;
    let settings = GameSettings::initialize()?;

    // let window = {
    //   let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    //   let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
    //   WindowBuilder::new()
    //     .with_title("Conway's Game of Life")
    //     .with_inner_size(scaled_size)
    //     .with_min_inner_size(size)
    //     .build(&event_loop)
    //     .unwrap()
    // };
    //
    // let mut pixels = {
    //   let window_size = window.inner_size();
    //   let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    //   Pixels::new(WIDTH, HEIGHT, surface_texture)?
    // };

    let game = GameConfig::new();
    let renderer = Renderer::new();

    Ok(Self {
      game,
      renderer,
      window,
      settings,
    })
  }

  pub fn run(self) -> anyhow::Result<()> {
    Ok(())
  }
}
