use crate::define_menu_items;

define_menu_items! {
  pub enum MainMenuItems {
    Start(item_name = "start", asset_name = "menu_start_v2"),
    Options(item_name = "options", asset_name = "menu_options"),
    Exit(item_name = "exit", asset_name = "menu_exit"),
  }
}
