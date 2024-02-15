use crate::game::game_config::GameConfig;
use crate::game::game_settings::GameSettings;
use crate::general_data::logging;
use crate::general_data::winit_traits::*;
use pixels::Pixels;
use pixels::SurfaceTexture;
use renderer::Renderer;
use winit::dpi::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const BASELINE_WINDOW_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
  width: 800,
  height: 1400,
};

pub struct RustrisConfig {
  game: GameConfig,
  renderer: Renderer,
  window: Window,
  event_loop: EventLoop<()>,
  settings: GameSettings,
}

// Square space: 50x50 pixels baseline. 1 pixel seperation

impl RustrisConfig {
  pub fn new() -> anyhow::Result<Self> {
    // The handle isn't required for current uses
    let _ = logging::setup_file_logger();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
      .with_title("Rustris")
      .with_inner_size(BASELINE_WINDOW_DIMENSIONS)
      .with_resizable(false)
      .build(&event_loop)?;
    let settings = GameSettings::initialize()?;

    let window_size = window.inner_size();
    let surface = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(window_size.width, window_size.height, surface)?;

    let game = GameConfig::new();
    let renderer = Renderer::new(pixels);

    Ok(Self {
      game,
      renderer,
      window,
      event_loop,
      settings,
    })
  }

  #[allow(unused)]
  pub fn run(mut self) -> anyhow::Result<()> {
    let mut game = self.game;
    let mut renderer = self.renderer;
    let window = self.window;
    let event_loop = self.event_loop;
    let mut game_settings = self.settings;
    let mut input = WinitInputHelper::new();

    let image_one = image::open("assets/test_image_physical.png")?;
    let image_two = image::open("assets/test_image_transparent.png")?;

    renderer.set_color([0, 0xFF, 0]);

    event_loop.run(|event, event_loop_window_target| {
      event_loop_window_target.set_control_flow(ControlFlow::Poll);

      if let Some(window_id) = event.get_window_id() {
        if window_id != window.id() {
          return;
        }
      }

      if let Event::WindowEvent {
        event: WindowEvent::RedrawRequested,
        ..
      } = event
      {
        println!("Redrawing screen.");
        if let Err(error) = renderer.complete_render() {
          println!("Failed to render to the frame buffer. `{:?}`", error);

          event_loop_window_target.exit();
        }

        // if let Err(error) = Self::render(&mut renderer, &game) {
        //   log::error!("Failed to render the screen: {:?}", error);
        //
        //   event_loop_window_target.exit();
        // }
      }

      if input.update(&event) {
        if let Err(error) = game.handle_input(&input) {
          log::error!("An error occurred when handling an input: {:?}", error);
        }

        window.request_redraw();
      }
    })?;

    Ok(())
  }

  fn render(renderer: &mut Renderer, game: &GameConfig) -> anyhow::Result<()> {
    todo!()
  }
}
