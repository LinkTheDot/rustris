use super::actions::{MenuAction, PlayerAction};
use super::minos::MinoType;
use crate::game::world_state::*;
use crate::menus::menu_data::*;
use crate::menus::templates::{game_settings::*, main_menu::*};
use crate::renderer::text_boxes::TextBox;
use crate::renderer::Renderer;
use crate::rustris_config::RENDERED_WINDOW_DIMENSIONS;
use anyhow::anyhow;
use maplit::hashmap;
use std::collections::HashMap;
use winit::dpi::*;

#[allow(unused)]
#[derive(Debug)]
pub struct WorldData {
  current_state: WorldState,

  held: Option<MinoType>,
  /// Contains the list of filled squares and the piece that occupies them.
  board: Vec<Option<MinoType>>,

  current_menu: Option<&'static str>,
  menus: HashMap<&'static str, Menu>,
  text_boxes: HashMap<&'static str, TextBox>,
}

impl WorldData {
  pub const LOGICAL_BOARD_WIDTH: u32 = 10;
  pub const LOGICAL_BOARD_HEIGHT: u32 = 40;
  pub const VISIBLE_BOARD_WIDTH: u32 = 10;
  pub const VISIBLE_BOARD_HEIGHT: u32 = 20;

  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    log::info!("Creating world data.");

    let menus = Self::load_menus();
    let text_boxes = Self::load_text_boxes();

    Self {
      current_state: WorldState::Menu,

      held: None,
      board: vec![None; Self::LOGICAL_BOARD_WIDTH as usize * Self::LOGICAL_BOARD_HEIGHT as usize],

      current_menu: Some(MainMenuItems::MENU_NAME),
      menus,
      text_boxes,
    }
  }

  fn load_menus() -> HashMap<&'static str, Menu> {
    hashmap! {
      MainMenuItems::MENU_NAME => Menu::new::<MainMenuItems>(),
      GeneralSettingsMenuItems::MENU_NAME => Menu::new::<GeneralSettingsMenuItems>(),
      GameControlsMenuItems::MENU_NAME => Menu::new::<GameControlsMenuItems>(),
      MenuControlsMenuItems::MENU_NAME => Menu::new::<MenuControlsMenuItems>(),

    }
  }

  fn load_text_boxes() -> HashMap<&'static str, TextBox> {
    let mut text_boxes = HashMap::new();
    let general_settings = GeneralSettingsMenuItems::create_text_boxes();
    let game_controls = GameControlsMenuItems::create_text_boxes();
    let menu_controls = MenuControlsMenuItems::create_text_boxes();

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

    let current_menu = self.current_menu_mut()?;

    match current_menu.name() {
      "main_menu" => match player_action {
        MenuAction::Up => current_menu.previous(),
        MenuAction::Down => current_menu.next(),
        MenuAction::Select => {
          let Some(current_option) = current_menu.current_option() else {
            return Err(anyhow!(
              "The current menu, `{}`, has no options.",
              current_menu.name()
            ));
          };

          match current_option.item_name() {
            "start" => self.update_state(WorldState::Game),
            "options" => self.current_menu = Some("options_menu"),
            "exit" => return Ok(true),
            _ => (),
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
      _ => (),
    }

    Ok(false)
  }

  fn update_game(&mut self, _player_action: Option<PlayerAction>) -> anyhow::Result<()> {
    Ok(())
  }

  pub fn render(&self, renderer: &mut Renderer) -> anyhow::Result<()> {
    match self.current_state {
      WorldState::Menu => {
        let current_menu_name = self.current_menu.unwrap_or("main_menu");

        match current_menu_name {
          MainMenuItems::MENU_NAME => self.render_main_menu(renderer)?,
          "options template" => self.render_options(renderer)?,
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

    let menu_position = LogicalPosition {
      x: 0,
      y: (RENDERED_WINDOW_DIMENSIONS.height as f32 * 0.25).cast::<i32>(),
    };
    let option_spacing = 20; // pixels.
    let current_menu = self.current_menu()?;

    #[allow(unused_labels)]
    'temp_testing: {
      let end_position = LogicalPosition::new(100, 50);
      let length = 30;
      let point_right = true;
      let color = [0xFF; 4];

      renderer.draw_arrow(&end_position, length, point_right, &color)?;

      // TODO!(): Drawing an arrow at the position of the selected menu item
      // let current_selected = current_menu.asset_name_at_index(current_menu.cursor_position());
      // let asset_position =
      //   current_menu.asset_position_at_index(renderer, current_menu.cursor_position());
    }

    current_menu.render_with_images(&menu_position, renderer, option_spacing)
  }

  fn render_options(&self, _renderer: &mut Renderer) -> anyhow::Result<()> {
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
