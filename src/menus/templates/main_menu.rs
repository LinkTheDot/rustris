use crate::define_menu_items;
use crate::rustris_config::RENDERED_WINDOW_DIMENSIONS;

define_menu_items! {
  pub const POSITION = LogicalPosition { x: RENDERED_WINDOW_DIMENSIONS.width / 2, y: (RENDERED_WINDOW_DIMENSIONS.height * 25) / 100 };
  pub const ITEM_OFFSET = 20;

  pub enum MainMenu {
    Start(item_name = "start", asset_name = "menu_start_v2"),
    Options(item_name = "options", asset_name = "menu_options"),
    Exit(item_name = "exit", asset_name = "menu_exit"),
  }
}
