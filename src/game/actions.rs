use super::world_state::WorldState;
use winit::keyboard::KeyCode;

/// The variations of player actions depending on the environment.
///
/// If the player is in the game, [`GameAction`](GameAction) will be used.
/// GameAction contains a list of actions that the player has enacted, such as movement and piece dropping.
///
/// If the player is in a menu (eg. pause menu, main menu, etc.). The [`MenuAction`](MenuAction) will be used.
/// MenuAction contains a single value which can be movement, menu selection, etc.
#[derive(Debug, Clone)]
pub enum PlayerAction {
  GameAction(Vec<GameAction>),
  MenuAction(MenuAction),
}

/// The list of actions that can be taken while playing the game.
///
/// These actions consist of piece movement, dropping style, pausing, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
  MoveLeft,
  MoveRight,
  HardDrop,
  SoftDrop,
  Hold,
  Pause,

  Unknown,
}

/// The list of actions that can be taken within a menu.
///
/// Menus consist of the main menu, settings menu, pause menu, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
  Up,
  Down,
  Left,
  Right,
  Select,
  Back,

  Unknown,
}

impl PlayerAction {
  /// Returns true if the input is either [`MenuAction::Unknown`](MenuAction), [`GameAction::Unknown`](GameAction), or GameAction with an empty list.
  pub fn is_empty(&self) -> bool {
    match self {
      PlayerAction::GameAction(action) => {
        if action.is_empty() {
          return true;
        }

        action.iter().all(GameAction::is_empty)
      }
      PlayerAction::MenuAction(action) => action.is_empty(),
    }
  }
}

impl GameAction {
  /// Returns true if the input is [`GameAction::Unknown`](GameAction).
  pub fn is_empty(&self) -> bool {
    self == &GameAction::Unknown
  }
}

impl MenuAction {
  /// Returns true if the input is [`MenuAction::Unknown`](MenuAction).
  pub fn is_empty(&self) -> bool {
    self == &MenuAction::Unknown
  }
}

// TODO: Make these compatible with changing keybindings in the options.

impl From<KeyCode> for GameAction {
  fn from(key: KeyCode) -> Self {
    match key {
      KeyCode::ArrowLeft | KeyCode::KeyA => GameAction::MoveLeft,
      KeyCode::ArrowRight | KeyCode::KeyD => GameAction::MoveRight,
      KeyCode::ArrowDown | KeyCode::KeyS => GameAction::SoftDrop,

      KeyCode::Space => GameAction::HardDrop,
      KeyCode::ArrowUp => GameAction::Hold,
      KeyCode::Escape => GameAction::Pause,

      _ => GameAction::Unknown,
    }
  }
}

impl From<KeyCode> for MenuAction {
  fn from(key: KeyCode) -> Self {
    match key {
      KeyCode::ArrowUp | KeyCode::KeyW => MenuAction::Up,
      KeyCode::ArrowDown | KeyCode::KeyS => MenuAction::Down,
      KeyCode::ArrowLeft | KeyCode::KeyA => MenuAction::Left,
      KeyCode::ArrowRight | KeyCode::KeyD => MenuAction::Right,

      KeyCode::Enter | KeyCode::KeyZ => MenuAction::Select,
      KeyCode::Backspace | KeyCode::KeyX | KeyCode::Escape => MenuAction::Back,

      _ => MenuAction::Unknown,
    }
  }
}

impl From<(WorldState, KeyCode)> for PlayerAction {
  fn from((world_state, key): (WorldState, KeyCode)) -> Self {
    match world_state {
      WorldState::Menu => PlayerAction::MenuAction(MenuAction::from(key)),
      WorldState::Game => PlayerAction::GameAction(vec![GameAction::from(key)]),
    }
  }
}

impl From<(WorldState, Vec<KeyCode>)> for PlayerAction {
  fn from((world_state, keys): (WorldState, Vec<KeyCode>)) -> Self {
    if keys.is_empty() {
      return PlayerAction::MenuAction(MenuAction::Unknown);
    }

    match world_state {
      WorldState::Menu => PlayerAction::MenuAction(MenuAction::from(keys[0])),
      WorldState::Game => keys
        .into_iter()
        .filter_map(|key| {
          let action = GameAction::from(key);

          if action != GameAction::Unknown {
            Some(action)
          } else {
            None
          }
        })
        .collect::<Vec<GameAction>>()
        .into(),
    }
  }
}

impl From<Vec<GameAction>> for PlayerAction {
  fn from(actions: Vec<GameAction>) -> Self {
    PlayerAction::GameAction(actions)
  }
}
