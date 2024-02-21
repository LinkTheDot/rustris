#![allow(unused)]

use crate::game::game_settings::GameSettings;
use crate::game::world_data::WorldData;
use crate::general_data::logging;
use crate::general_data::result_traits::*;
use crate::general_data::winit_traits::*;
use game_loop::GameLoop;
use game_loop::{game_loop, Time, TimeTrait as _};
use pixels::Pixels;
use pixels::SurfaceTexture;
use renderer::Renderer;
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::time::Duration;
use winit::dpi::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub const FPS: usize = 1000;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

const BASELINE_WINDOW_DIMENSIONS: PhysicalSize<u32> = PhysicalSize {
  width: 500,
  height: 800,
};

pub struct RustrisConfig {
  world_data: WorldData,
  renderer: Renderer,
  settings: GameSettings,
  window: Arc<Window>,
  input: WinitInputHelper,
}

// Square space: 30x30 pixels baseline, 1 pixel seperation

impl RustrisConfig {
  pub fn new() -> anyhow::Result<(Self, EventLoop<()>)> {
    // The handle isn't required for current uses
    let _ = logging::setup_file_logger();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
      .with_title("Rustris")
      .with_inner_size(BASELINE_WINDOW_DIMENSIONS)
      .with_resizable(false)
      .build(&event_loop)?;
    let window = Arc::new(window);
    let window_size = window.inner_size();

    let surface = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(window_size.width, window_size.height, surface)?;

    let settings = GameSettings::initialize()?;
    let input = WinitInputHelper::new();

    let game = WorldData::new();
    let renderer = Renderer::new(pixels);

    Ok((
      Self {
        world_data: game,
        renderer,
        settings,
        window,
        input,
      },
      event_loop,
    ))
  }

  #[allow(unused)]
  pub fn run(mut self, event_loop: EventLoop<()>) -> anyhow::Result<()> {
    let fps = self.settings.fps();

    // const PIXELS_PER_MOVEMENT: i32 = 5;
    // self.renderer.set_color([0, 0, 0])?;

    game_loop(
      event_loop,
      self.window.clone(),
      self,
      // fps,
      FPS as u32,
      0.1,
      Self::update_game,
      Self::render,
      Self::handle_winit_events,
    )?;

    Ok(())
  }

  fn update_game(game_loop: &mut GameLoop<Self, Time, Arc<Window>>) {
    // todo!()
  }

  #[allow(clippy::needless_return)]
  fn render(game_loop: &mut GameLoop<Self, Time, Arc<Window>>) {
    // game_loop.game.renderer.set_color([0, 0, 0])
    if let Err(error) = game_loop.game.renderer.clear() {
      log::error!("Failed to render to the frame buffer. `{:?}`", error);

      game_loop.exit();

      return;
    }

    // TODO: implement drawing the world.

    // TEMP
    let position_one = PhysicalPosition { x: 100, y: 100 };
    let position_two = PhysicalPosition { x: 105, y: 105 };
    let dimensions = PhysicalSize {
      width: 40,
      height: 40,
    };
    let window_dimensions = game_loop.game.window.inner_size();
    let buffer = game_loop.game.renderer.pixels.frame_mut();

    Renderer::draw_rectangle(
      buffer,
      position_one,
      dimensions,
      [255, 0, 0, 255],
      &window_dimensions,
    )
    .log_if_err("Failed to draw rectangle: ");
    Renderer::draw_rectangle(
      buffer,
      position_two,
      dimensions,
      [0, 0, 255, 117],
      &window_dimensions,
    )
    .log_if_err("Failed to draw rectangle: ");
    // TEMP

    if let Err(error) = game_loop.game.renderer.complete_render() {
      log::error!("Failed to render to the frame buffer. '{:?}'", error);

      game_loop.exit();

      return;
    }

    // let time_step = Duration::from_nanos(1_000_000_000 / game_loop.game.settings.fps() as u64);
    // let dt = time_step.as_secs_f64() - Time::now().sub(&game_loop.current_instant());

    let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&game_loop.current_instant());
    if dt > 0.0 {
      // Sleep the main thread to limit drawing to the fixed time step.
      // See: https://github.com/parasyte/pixels/issues/174
      std::thread::sleep(Duration::from_secs_f64(dt));
    }
  }

  #[allow(clippy::needless_return)]
  fn handle_winit_events(game_loop: &mut GameLoop<Self, Time, Arc<Window>>, event: &Event<()>) {
    if game_loop.game.input.update(event) {
      if game_loop.game.input.key_pressed(KeyCode::Escape) || game_loop.game.input.close_requested()
      {
        game_loop.exit();
        return;
      }

      if let Some(new_dimensions) = game_loop.game.input.window_resized() {
        if let Err(error) = game_loop.game.renderer.resize_surface(new_dimensions) {
          log::error!("Failed to change surface dimensions: '{:?}'", error);

          game_loop.exit();

          return;
        }
      }
    }
  }
}
