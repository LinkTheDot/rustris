use crate::menus::menu_items::MenuItemData;
use crate::{define_menu_items, renderer::text_boxes::TextBox};

define_menu_items! {
  pub const POSITION = LogicalPosition { x: 0, y: 0 };
  pub const ITEM_OFFSET = 10;

  pub enum GeneralSettingsMenu {
    Fps(item_name = "fps", asset_name = "unknown"),
  }
}

define_menu_items! {
  pub const POSITION = LogicalPosition { x: 0, y: 0 };
  pub const ITEM_OFFSET = 10;

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
  pub const POSITION = LogicalPosition { x: 0, y: 0 };
  pub const ITEM_OFFSET = 10;

  pub enum MenuControlsMenu {
    Up(item_name = "move_up", asset_name = "move_up_menu_control_text_box"),
    Down(item_name = "move_down", asset_name = "move_down_menu_control_text_box"),
    Left(item_name = "move_left", asset_name = "move_left_menu_control_text_box"),
    Right(item_name = "move_right", asset_name = "move_right_menu_control_text_box"),
    Select(item_name = "select", asset_name = "select_menu_control_text_box"),
    Back(item_name = "back", asset_name = "back_menu_control_text_box"),
  }
}

impl GeneralSettingsMenu {
  pub fn create_text_boxes() -> Vec<(&'static str, TextBox)> {
    let font_index = 0;
    let font_size = 20.0;
    let text_gap = 3;

    let text_box_list = vec![(Self::Fps.asset_name(), "FPS:")];

    TextBox::new_set_from_list(
      font_index,
      font_size,
      text_gap,
      Self::POSITION,
      text_box_list,
    )
  }
}

impl GameControlsMenu {
  pub fn create_text_boxes() -> Vec<(&'static str, TextBox)> {
    let font_index = 0;
    let font_size = 20.0;
    let text_gap = 3;

    let text_box_list = vec![
      (Self::MoveLeft.asset_name(), "Left piece movement:"),
      (Self::MoveRight.asset_name(), "Right piece movement:"),
      (Self::HardDrop.asset_name(), "Hard drop:"),
      (Self::SoftDrop.asset_name(), "Soft drop:"),
      (Self::HoldPiece.asset_name(), "Hold piece:"),
      (Self::Pause.asset_name(), "Pause:"),
    ];

    TextBox::new_set_from_list(
      font_index,
      font_size,
      text_gap,
      Self::POSITION,
      text_box_list,
    )
  }
}

impl MenuControlsMenu {
  pub fn create_text_boxes() -> Vec<(&'static str, TextBox)> {
    let font_index = 0;
    let font_size = 20.0;
    let text_gap = 3;

    let text_box_list = vec![
      (Self::Up.asset_name(), "Move cursor up: "),
      (Self::Down.asset_name(), "Move cursor down: "),
      (Self::Left.asset_name(), "Move cursor left: "),
      (Self::Right.asset_name(), "Move cursor right: "),
      (Self::Select.asset_name(), "Select at cursor: "),
      (Self::Back.asset_name(), "Back: "),
    ];

    TextBox::new_set_from_list(
      font_index,
      font_size,
      text_gap,
      Self::POSITION,
      text_box_list,
    )
  }
}
