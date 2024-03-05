use crate::{
  define_menu_items,
  menus::{menu_data::Menu, menu_items::*},
};

pub struct MainMenu;

impl MainMenu {
  pub const MENU_NAME: &'static str = "main_menu";

  pub fn new_menu() -> Menu {
    let menu_name = Self::MENU_NAME;

    Menu::new::<MainMenuItems>(menu_name)
  }
}

define_menu_items! {
  pub enum MainMenuItems {
    Start(item_name = "start", asset_name = "menu_start_v2"),
    Options(item_name = "options", asset_name = "menu_options"),
    Exit(item_name = "exit", asset_name = "menu_exit"),
  }
}
