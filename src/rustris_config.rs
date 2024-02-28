use crate::game::{actions::*, game_settings::GameSettings, world_data::WorldData};
use crate::general_data::winit_traits::*;
use game_loop::{game_loop, GameLoop, Time, TimeTrait};
use pixels::{Pixels, SurfaceTexture};
use renderer::Renderer;
use std::sync::Arc;
use std::time::Duration;
use winit::window::{Window, WindowBuilder};
use winit::{dpi::*, event::Event, event_loop::EventLoop, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

pub const RENDERED_WINDOW_DIMENSIONS: LogicalSize<u32> = LogicalSize::new(250, 400);

pub struct RustrisConfig {
  world_data: WorldData,
  player_action: Option<PlayerAction>,
  renderer: Renderer,
  settings: GameSettings,
  input: WinitInputHelper,
}

impl RustrisConfig {
  pub fn new() -> anyhow::Result<(Self, EventLoop<()>, Window)> {
    let event_loop = EventLoop::new()?;

    let primary_monitor_dimensions = get_primary_monitor_dimensions(&event_loop);
    let window_scale =
      (primary_monitor_dimensions.height / RENDERED_WINDOW_DIMENSIONS.height).max(1);
    let scaled_window_dimensions = RENDERED_WINDOW_DIMENSIONS.multiply(window_scale);

    log::info!("window scale: {:?}", window_scale);

    let window = WindowBuilder::new()
      .with_title("Rustris")
      .with_inner_size(scaled_window_dimensions)
      .with_min_inner_size(RENDERED_WINDOW_DIMENSIONS)
      .build(&event_loop)?;
    let window_size = window.inner_size();

    let surface = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(
      RENDERED_WINDOW_DIMENSIONS.width,
      RENDERED_WINDOW_DIMENSIONS.height,
      surface,
    )?;

    let settings = GameSettings::initialize()?;
    let input = WinitInputHelper::new();

    let game = WorldData::new();
    let renderer = Renderer::new(pixels);

    let rustris_config = Self {
      world_data: game,
      player_action: None,
      renderer,
      settings,
      input,
    };

    Ok((rustris_config, event_loop, window))
  }

  pub fn run(self, event_loop: EventLoop<()>, window: Window) -> anyhow::Result<()> {
    let window = Arc::new(window);
    let fps = self.settings.fps();

    game_loop(
      event_loop,
      window,
      self,
      fps,
      0.1,
      Self::update_game,
      Self::render,
      Self::handle_winit_events,
    )?;

    Ok(())
  }

  fn update_game(game_loop: &mut GameLoop<Self, Time, Arc<Window>>) {
    if let Err(error) = game_loop
      .game
      .world_data
      .update_world(game_loop.game.player_action.clone())
    {
      log::error!("An error occurred when updating the world: {:?}", error);

      game_loop.exit();

      return;
    }

    if game_loop.game.settings.fps() != game_loop.updates_per_second {
      game_loop.set_updates_per_second(game_loop.game.settings.fps());
    }
  }

  fn render(game_loop: &mut GameLoop<Self, Time, Arc<Window>>) {
    if let Err(error) = game_loop.game.renderer.clear() {
      log::error!("Failed to render to clear the frame buffer. `{:?}`", error);

      game_loop.exit();

      return;
    }

    if let Err(error) = game_loop
      .game
      .world_data
      .render(&mut game_loop.game.renderer)
    {
      log::error!("Failed to render the game world: `{:?}`", error);
    }

    if let Err(error) = game_loop.game.renderer.complete_render() {
      log::error!("Failed to render to the frame buffer. '{:?}'", error);

      game_loop.exit();

      return;
    }

    let fps = game_loop.game.settings.fps() as f64;
    let time_step = 1.0 / fps;
    let delta_time = time_step - Time::now().sub(&game_loop.current_instant());

    if delta_time > 0.0 {
      // Sleep the main thread to limit drawing to the fixed time step.
      // https://github.com/parasyte/pixels/issues/174
      std::thread::sleep(Duration::from_secs_f64(delta_time));
    }
  }

  #[allow(clippy::needless_return)]
  fn handle_winit_events(game_loop: &mut GameLoop<Self, Time, Arc<Window>>, event: &Event<()>) {
    if !game_loop.game.input.update(event) {
      return;
    }

    if game_loop.game.input.close_requested() {
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

    game_loop.game.update_input(event);
  }

  fn update_input(&mut self, event: &Event<()>) {
    // This will change once keybind settings are implemented.
    const TEMP_VALID_KEYS: &[KeyCode] = &[
      KeyCode::ArrowLeft,
      KeyCode::ArrowRight,
      KeyCode::ArrowUp,
      KeyCode::ArrowDown,
      KeyCode::Space,
      KeyCode::Escape,
      KeyCode::Enter,
      KeyCode::Backspace,
      KeyCode::KeyW,
      KeyCode::KeyA,
      KeyCode::KeyS,
      KeyCode::KeyD,
    ];

    if self.input.update(event) {
      let world_state = self.world_data.world_state();
      let input = &self.input;

      let keys_pressed: Vec<KeyCode> = TEMP_VALID_KEYS
        .to_owned()
        .iter()
        .filter_map(|key| input.key_pressed(*key).then_some(*key))
        .collect();

      let player_action = PlayerAction::from((world_state, keys_pressed));

      if !player_action.is_empty() {
        self.player_action = Some(player_action)
      } else {
        self.player_action = None
      }
    }
  }
}

fn get_primary_monitor_dimensions(event_loop: &EventLoop<()>) -> PhysicalSize<u32> {
  let Some(primary_monitor) = event_loop.primary_monitor() else {
    return RENDERED_WINDOW_DIMENSIONS.to_physical(1.0);
  };

  primary_monitor.size()
}
