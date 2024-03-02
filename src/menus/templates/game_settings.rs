use crate::menus::menu_data::*;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref SETTINGS_MENU: Menu = {
    let options = vec![SettingsMenuOptions::Fps];

    Menu::new("settings_menu", options)
  };
}

pub enum SettingsMenuOptions {
  Fps,
}

impl MenuOptionData for SettingsMenuOptions {
  fn asset_name(&self) -> &'static str {
    match self {
      SettingsMenuOptions::Fps => "unknown",
    }
  }

  fn option_name(&self) -> &'static str {
    match self {
      SettingsMenuOptions::Fps => "fps",
    }
  }
}

impl From<SettingsMenuOptions> for MenuOption {
  fn from(item: SettingsMenuOptions) -> MenuOption {
    MenuOption::new(item)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameControlsMenuOptions {
  MoveLeft,
  MoveRight,
  HardDrop,
  SoftDrop,
  HoldPiece,
  Pause,
}

impl MenuOptionData for GameControlsMenuOptions {
  fn asset_name(&self) -> &'static str {
    match self {
      GameControlsMenuOptions::MoveLeft => "move_left_game_option_text",
      GameControlsMenuOptions::MoveRight => "move_right_game_option_text",
      GameControlsMenuOptions::HardDrop => "hard_drop_game_option_text",
      GameControlsMenuOptions::SoftDrop => "soft_drop_game_option_text",
      GameControlsMenuOptions::HoldPiece => "hold_piece_game_option_text",
      GameControlsMenuOptions::Pause => "pause_game_option_text",
    }
  }

  fn option_name(&self) -> &'static str {
    match self {
      GameControlsMenuOptions::MoveLeft => "move_left",
      GameControlsMenuOptions::MoveRight => "move_right",
      GameControlsMenuOptions::HardDrop => "hard_drop",
      GameControlsMenuOptions::SoftDrop => "soft_drop",
      GameControlsMenuOptions::HoldPiece => "hold_piece",
      GameControlsMenuOptions::Pause => "pause",
    }
  }
}

impl From<GameControlsMenuOptions> for MenuOption {
  fn from(item: GameControlsMenuOptions) -> MenuOption {
    MenuOption::new(item)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuControlsMenuOptions {
  Up,
  Down,
  Left,
  Right,
  Select,
  Back,
}

impl MenuOptionData for MenuControlsMenuOptions {
  fn asset_name(&self) -> &'static str {
    match self {
      MenuControlsMenuOptions::Up => "move_up_menu_option_text",
      MenuControlsMenuOptions::Down => "move_down_menu_option_text",
      MenuControlsMenuOptions::Left => "move_left_menu_option_text",
      MenuControlsMenuOptions::Right => "move_right_menu_option_text",
      MenuControlsMenuOptions::Select => "select_menu_option_text",
      MenuControlsMenuOptions::Back => "back_menu_option_text",
    }
  }

  fn option_name(&self) -> &'static str {
    match self {
      MenuControlsMenuOptions::Up => "move_up",
      MenuControlsMenuOptions::Down => "move_down",
      MenuControlsMenuOptions::Left => "move_left",
      MenuControlsMenuOptions::Right => "move_right",
      MenuControlsMenuOptions::Select => "select",
      MenuControlsMenuOptions::Back => "back",
    }
  }
}

impl From<MenuControlsMenuOptions> for MenuOption {
  fn from(item: MenuControlsMenuOptions) -> MenuOption {
    MenuOption::new(item)
  }
}
