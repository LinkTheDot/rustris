use crate::menus::menu_data::*;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref MAIN_MENU: Menu = {
    let options = vec![
      MainMenuOptions::Start,
      MainMenuOptions::Options,
      MainMenuOptions::Exit,
    ];

    Menu::new("main_menu", options)
  };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuOptions {
  Start,
  Options,
  Exit,
}

impl MenuOptionData for MainMenuOptions {
  fn asset_name(&self) -> &'static str {
    match self {
      MainMenuOptions::Start => "menu_start_v2",
      MainMenuOptions::Options => "menu_options",
      MainMenuOptions::Exit => "menu_exit",
    }
  }

  fn option_name(&self) -> &'static str {
    match self {
      MainMenuOptions::Start => "start",
      MainMenuOptions::Options => "options",
      MainMenuOptions::Exit => "exit",
    }
  }
}

impl From<MainMenuOptions> for MenuOption {
  fn from(item: MainMenuOptions) -> MenuOption {
    MenuOption::new(item)
  }
}
