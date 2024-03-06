use crate::define_menu_items;

define_menu_items! {
  pub enum GeneralSettingsMenuItems {
    Fps(item_name = "fps", asset_name = "unknown"),
  }
}

define_menu_items! {
  pub enum GameControlsMenu {
    MoveLeft(item_name = "move_left", asset_name = "move_left_game_control_text_box"),
    MoveRight(item_name = "move_right", asset_name = "move_right_game_control_text_box"),
    HardDrop(item_name = "hard_drop", asset_name = "hard_drop_game_control_text_box"),
    SoftDrop(item_name = "soft_drop", asset_name = "soft_drop_game_control_text_box"),
    HoldPiece(item_name = "hold_piece", asset_name = "hold_piece_game_control_text_box"),
    Pause(item_name = "pause", asset_name = "pause_game_control_text_box"),
  }
}

define_menu_items! {
  pub enum MenuControlsMenuItems {
    Up(item_name = "move_up", asset_name = "move_up_menu_control_text_box"),
    Down(item_name = "move_down", asset_name = "move_down_menu_control_text_box"),
    Left(item_name = "move_left", asset_name = "move_left_menu_control_text_box"),
    Right(item_name = "move_right", asset_name = "move_right_menu_control_text_box"),
    Select(item_name = "select", asset_name = "select_menu_control_text_box"),
    Back(item_name = "back", asset_name = "back_menu_control_text_box"),
  }
}
