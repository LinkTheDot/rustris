use crate::{define_menu_items, menus::menu_data::*, menus::menu_items::*};
// use lazy_static::lazy_static;

pub struct Settings;

impl Settings {
  pub const GENERAL_SETTINGS_NAME: &'static str = "settings_menu";
  pub const GAME_CONTROLS_NAME: &'static str = "game_controls";
  pub const MENU_CONTROLS_NAME: &'static str = "menu_controls";

  pub fn general_settings_menu() -> Menu {
    Menu::new::<GeneralSettingsMenuItems>(Self::GENERAL_SETTINGS_NAME)
  }

  pub fn game_controls_menu() -> Menu {
    Menu::new::<MenuControlsMenuItems>(Self::GAME_CONTROLS_NAME)
  }

  pub fn menu_controls_menu() -> Menu {
    Menu::new::<MenuControlsMenuItems>(Self::MENU_CONTROLS_NAME)
  }
}

define_menu_items! {
  pub enum GeneralSettingsMenuItems {
    Fps(item_name = "fps", asset_name = "unknown"),
  }
}

define_menu_items! {
  pub enum GameControlsMenu {
    MoveLeft(item_name = "move_left", asset_name = "move_left_game_option_text"),
    MoveRight(item_name = "move_right", asset_name = "move_right_game_option_text"),
    HardDrop(item_name = "hard_drop", asset_name = "hard_drop_game_option_text"),
    SoftDrop(item_name = "soft_drop", asset_name = "soft_drop_game_option_text"),
    HoldPiece(item_name = "hold_piece", asset_name = "hold_piece_game_option_text"),
    Pause(item_name = "pause", asset_name = "pause_game_option_text"),
  }
}

define_menu_items! {
  pub enum MenuControlsMenuItems {
    Up(item_name = "move_up", asset_name = "move_up_menu_option_text"),
    Down(item_name = "move_down", asset_name = "move_down_menu_option_text"),
    Left(item_name = "move_left", asset_name = "move_left_menu_option_text"),
    Right(item_name = "move_right", asset_name = "move_right_menu_option_text"),
    Select(item_name = "select", asset_name = "select_menu_option_text"),
    Back(item_name = "back", asset_name = "back_menu_option_text"),
  }
}
