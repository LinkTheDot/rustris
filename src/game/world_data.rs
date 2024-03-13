use super::minos::MinoType;
use crate::game::actions::{MenuAction, PlayerAction};
use crate::game::game_settings::GameSettings;
use crate::game::timer::Timer;
use crate::game::world_state::*;
use crate::get_renderable_from_name;
use crate::menus::menu_data::*;
use crate::menus::templates::{game_settings::*, main_menu::*};
use crate::renderer::text_boxes::TextBox;
use crate::renderer::Renderer;
use crate::rustris_config::RENDERED_WINDOW_DIMENSIONS;
use anyhow::anyhow;
use maplit::hashmap;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;
use winit::dpi::*;

static TEXT_BOXES: OnceLock<HashMap<&'static str, TextBox>> = OnceLock::new();

#[allow(unused)]
#[derive(Debug)]
pub struct WorldData {
  current_state: WorldState,

  held: Option<MinoType>,
  /// Contains the list of filled squares and the piece that occupies them.
  ///
  /// Size is [`logical_width`](WorldData::LOGICAL_BOARD_WIDTH) * [`logical_height`](WorldData::LOGICAL_BOARD_HEIGHT)
  board: Vec<Option<MinoType>>,

  current_menu: Option<&'static str>,
  menus: HashMap<&'static str, Menu>,

  timers: HashMap<&'static str, Timer>,
}

impl WorldData {
  /// The width of the board when calculating game interactions.
  pub const LOGICAL_BOARD_WIDTH: u32 = 10;
  /// The height of the board when calculating game interactions.
  pub const LOGICAL_BOARD_HEIGHT: u32 = 40;

  /// The width of the board when rendering it.
  pub const VISIBLE_BOARD_WIDTH: u32 = 10;
  /// The height of the board when rendering it.
  pub const VISIBLE_BOARD_HEIGHT: u32 = 20;

  #[allow(clippy::new_without_default)]
  pub fn new() -> anyhow::Result<Self> {
    log::info!("Creating world data.");

    let menus = Self::load_menus()?;
    let timers = HashMap::new();

    Ok(Self {
      current_state: WorldState::Menu,

      held: None,
      board: vec![None; Self::LOGICAL_BOARD_WIDTH as usize * Self::LOGICAL_BOARD_HEIGHT as usize],

      current_menu: Some(MainMenu::MENU_NAME),
      menus,

      timers,
    })
  }

  fn load_menus() -> anyhow::Result<HashMap<&'static str, Menu>> {
    Ok(hashmap! {
      MainMenu::MENU_NAME => Menu::new::<MainMenu>()?,
      GeneralSettingsMenu::MENU_NAME => Menu::new::<GeneralSettingsMenu>()?,
      GameControlsMenu::MENU_NAME => Menu::new::<GameControlsMenu>()?,
      MenuControlsMenu::MENU_NAME => Menu::new::<MenuControlsMenu>()?,
    })
  }

  pub fn get_text_boxes() -> &'static HashMap<&'static str, TextBox> {
    TEXT_BOXES.get_or_init(WorldData::initialize_text_boxes)
  }

  /// Gets the timer of the given name.
  ///
  /// If it doesn't exist, it's initialized with the passed in duration.
  ///
  /// If the name and duration don't exist, 10 seconds is defaulted as the timer.
  pub fn get_or_init_timer(&mut self, name: &'static str, duration: Option<Duration>) -> &Timer {
    let duration = duration.unwrap_or(Duration::from_secs(10));

    self.timers.entry(name).or_insert(Timer::new(duration))
  }

  /// Gets the timer of the given name.
  ///
  /// None is returned if the timer doesn't exist.
  /// Use [`.get_or_init_timer()`](WorldData::get_or_init_timer) for creating a timer.
  pub fn get_timer(&self, name: &'static str) -> Option<&Timer> {
    self.timers.get(name)
  }

  fn initialize_text_boxes() -> HashMap<&'static str, TextBox> {
    let mut text_boxes = HashMap::new();
    let general_settings = GeneralSettingsMenu::create_text_boxes();
    let game_controls = GameControlsMenu::create_text_boxes();
    let menu_controls = MenuControlsMenu::create_text_boxes();

    general_settings
      .into_iter()
      .chain(game_controls)
      .chain(menu_controls)
      .for_each(|(k, v)| {
        text_boxes.insert(k, v);
      });

    text_boxes
  }

  /// True is returned when a request to close the program was made.
  pub fn update_world(&mut self, player_action: Option<PlayerAction>) -> anyhow::Result<bool> {
    match self.current_state {
      WorldState::Menu => return self.update_menu(player_action),
      WorldState::Game => self.update_game(player_action)?,
    };

    Ok(false)
  }

  /// True is returned when a request to close the program was made.
  fn update_menu(&mut self, player_action: Option<PlayerAction>) -> anyhow::Result<bool> {
    let Some(PlayerAction::MenuAction(player_action)) = player_action else {
      return Ok(false);
    };

    log::debug!("Action taken: {:?}", player_action);

    let current_menu_name = self.current_menu()?.name();

    match current_menu_name {
      MainMenu::MENU_NAME => match player_action {
        MenuAction::Up | MenuAction::Down => {
          let timer = self.get_or_init_timer("menu_movement", Some(Duration::from_millis(200)));

          if timer.is_finished() || !timer.running() {
            timer.start();

            match player_action {
              MenuAction::Up => self.current_menu_mut()?.previous(),
              MenuAction::Down => self.current_menu_mut()?.next(),
              _ => (),
            }
          }
        }

        MenuAction::Select => {
          let Some(current_option) = self.current_menu()?.current_option() else {
            return Err(anyhow!(
              "The current menu, `{}`, has no options.",
              current_menu_name
            ));
          };

          let Some(current_option_item) = MainMenu::from_name(current_option.item_name()) else {
            return Err(anyhow!(
              "Current option {:?} has an invalid menu item name {:?}",
              current_menu_name,
              current_option
            ));
          };

          match current_option_item {
            MainMenu::Start => self.update_state(WorldState::Game),
            MainMenu::Options => self.current_menu = Some("options_menu"), // Change this to not a string literal
            MainMenu::Exit => return Ok(true),
          }
        }
        _ => (),
      },

      "options_menu" => {
        todo!()
      }

      "pause_menu" => {
        todo!()
      }
      _ => {
        log::error!("Unknown menu labeled in the game config, going back to main menu.");

        self.current_menu = Some(MainMenu::MENU_NAME);
      }
    }

    Ok(false)
  }

  fn update_game(&mut self, _player_action: Option<PlayerAction>) -> anyhow::Result<()> {
    Ok(())
  }

  pub fn render(
    &self,
    renderer: &mut Renderer,
    game_settings: &GameSettings,
  ) -> anyhow::Result<()> {
    match self.current_state {
      WorldState::Menu => {
        let current_menu_name = self.current_menu.unwrap_or("main_menu");

        match current_menu_name {
          MainMenu::MENU_NAME => self.render_main_menu(renderer)?,
          "options template" => self.render_options(renderer, game_settings)?,
          "pause_menu template" => {
            self.render_game(renderer)?;

            renderer.apply_color([0, 0, 0, 0x77])?;

            self.render_pause_screen(renderer)?;
          }
          _ => {
            return Err(anyhow!(
              "Attempted to load an unknown menu: `{current_menu_name}`"
            ))
          }
        }
      }

      WorldState::Game => self.render_game(renderer)?,
    }

    Ok(())
  }

  fn render_game(&self, _renderer: &mut Renderer) -> anyhow::Result<()> {
    todo!()
  }

  fn render_main_menu(&self, renderer: &mut Renderer) -> anyhow::Result<()> {
    draw_background_gradiant(renderer)?;

    let current_menu = self.current_menu()?;

    current_menu.render(renderer, None)?;

    self.draw_menu_selection_indicator(renderer)
  }

  fn render_options(
    &self,
    _renderer: &mut Renderer,
    _game_settings: &GameSettings,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn render_pause_screen(&self, _renderer: &mut Renderer) -> anyhow::Result<()> {
    todo!()
  }

  pub fn world_state(&self) -> WorldState {
    self.current_state
  }

  fn update_state(&mut self, new_state: WorldState) {
    self.current_state = new_state;
  }

  /// Returns a reference to the currently selected menu.
  ///
  /// # Errors
  ///
  /// - When there is no selected menu.
  /// - When the selected menu doesn't exist in the list of menus.
  fn current_menu(&self) -> anyhow::Result<&Menu> {
    let Some(current_menu_name) = self.current_menu else {
      return Err(anyhow!(
        "Attempted to get the current menu when there wasn't one."
      ));
    };
    match self.menus.get(current_menu_name) {
      Some(menu) => Ok(menu),
      None => Err(anyhow!("Currently selected menu does not exist.")),
    }
  }

  /// Returns a mutable reference to the currently selected menu.
  ///
  /// # Errors
  ///
  /// - When there is no selected menu.
  /// - When the selected menu doesn't exist in the list of menus.
  fn current_menu_mut(&mut self) -> anyhow::Result<&mut Menu> {
    let Some(current_menu_name) = self.current_menu else {
      return Err(anyhow!(
        "Attempted to get the current menu when there wasn't one."
      ));
    };
    match self.menus.get_mut(current_menu_name) {
      Some(menu) => Ok(menu),
      None => Err(anyhow!("Currently selected menu does not exist.")),
    }
  }

  pub fn draw_menu_selection_indicator(&self, renderer: &mut Renderer) -> anyhow::Result<()> {
    let current_menu = self.current_menu()?;

    let cursor_position = current_menu.cursor_position();

    let Some(current_selected_asset_name) = current_menu.asset_name_at_index(cursor_position)
    else {
      return Err(anyhow!(
        "Failed to get asset at index {} from menu {:?}",
        cursor_position,
        current_menu.name()
      ));
    };
    let current_selected_asset_position = current_menu
      .asset_position_at_index(cursor_position)
      .unwrap();

    let Some(current_selected_asset) = get_renderable_from_name(current_selected_asset_name) else {
      return Err(anyhow!(
        "Failed to get asset of name {:?}",
        current_selected_asset_name
      ));
    };

    let LogicalSize {
      height: asset_height,
      ..
    } = current_selected_asset.dimensions();

    let end_position = LogicalPosition {
      x: current_selected_asset_position.x - 10,
      y: current_selected_asset_position.y + (asset_height / 2),
    };
    let length = 20;
    let point_right = true;
    let color = [0xFF; 4];

    renderer.draw_arrow(&end_position, length, point_right, &color)?;

    Ok(())
  }
}

fn draw_background_gradiant(renderer: &mut Renderer) -> anyhow::Result<()> {
  let pixel_buffer = renderer.frame_mut();
  let buffer_dimensions = RENDERED_WINDOW_DIMENSIONS;
  let pixel_count = buffer_dimensions.width * buffer_dimensions.height;

  for index in 0..pixel_count {
    let (x, y) = (
      index % buffer_dimensions.width,
      index / buffer_dimensions.height,
    );

    let x_percentage = x as f64 / buffer_dimensions.width as f64;
    let y_percentage = y as f64 / buffer_dimensions.height as f64;

    let red = (255.0 * y_percentage).cast::<u8>();
    let blue = (255.0 * x_percentage).cast::<u8>();

    Renderer::draw_at_pixel_with_rgb(pixel_buffer, index as usize, &[red, 0, blue])?;
  }

  Ok(())
}
