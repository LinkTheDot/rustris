pub use crate::menus::menu_option::*;
use crate::{asset_loader::ASSETS, rustris_config::RENDERED_WINDOW_DIMENSIONS};
use anyhow::anyhow;
use image::GenericImageView;
use renderer::*;
use winit::dpi::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
  name: &'static str,
  /// The index for which option is currently selected.
  selected: usize,
  options: Vec<MenuOption>,
}

/// This trait will label the options for a menu.
///
/// Each option will have a way to convert into the name of its button asset.
pub trait MenuOptionData: Into<MenuOption> {
  fn asset_name(&self) -> &'static str {
    "unknown"
  }

  fn option_name(&self) -> &'static str {
    "unknown"
  }
}

impl Menu {
  /// Creates a new menu from a list of options.
  ///
  /// Each option will implement [`MenuOptionData`](MenuOptionData).
  /// This will force each option to know its name, and what asset it's tied to.
  /// This allows for better organization of the possible options in a menu.
  pub fn new(name: &'static str, options: Vec<impl MenuOptionData>) -> Self {
    let options: Vec<MenuOption> = options.into_iter().map(Into::into).collect();

    Self {
      name,
      selected: 0,
      options,
    }
  }

  /// Returns the assigned name of this menu.
  pub fn name(&self) -> &'static str {
    self.name
  }

  /// Returns the index of which menu option is selected at the moment.
  pub fn cursor_position(&self) -> usize {
    self.selected
  }

  /// Moves the cursor to the previous option, wrapping to the last option if the cursor is < 0.
  pub fn previous(&mut self) {
    let option_count = self.options.len();

    if option_count == 0 {
      return;
    }

    if self.selected == 0 {
      self.selected = option_count - 1;
    } else {
      self.selected -= 1;
    }
  }

  /// Moves the cursor to the next option, wrapping back around to the first option
  /// if it exceeds the amount of options.
  pub fn next(&mut self) {
    let option_count = self.options.len();

    if option_count == 0 {
      return;
    }

    if self.selected == option_count - 1 {
      self.selected = 0;
    } else {
      self.selected += 1
    }
  }

  /// Returns the currently selected menu option.
  ///
  /// Returns None if the list is empty.
  pub fn current_option(&self) -> Option<&MenuOption> {
    if self.options.is_empty() {
      return None;
    }

    let selected_option = self.options.get(self.selected);

    if selected_option.is_none() {
      log::error!(
        "Attempted to index a menu option that doesn't exist. Index: {}, Max: {}",
        self.selected,
        self.options.len()
      );

      return self.options.get(0);
    }

    selected_option
  }

  /// Renders the menu to the buffer with the given offset and option spacing.
  ///
  /// The option_spacing is the gap between each option in pixels, not the space between the center of each image.
  pub fn render(
    &self,
    position: &LogicalPosition<i32>,
    renderer: &mut Renderer,
    option_spacing: u32,
  ) -> anyhow::Result<()> {
    let mut previous_option_bottom = position.y as u32;

    for menu_option in self.options.iter() {
      let Some(image_asset) = ASSETS.get(menu_option.asset_name()) else {
        return Err(anyhow!("Failed to load asset {}", menu_option.asset_name()));
      };
      let (image_width, image_height) = image_asset.dimensions();

      let position = LogicalPosition {
        x: (((RENDERED_WINDOW_DIMENSIONS.width / 2) - (image_width / 2)) as i32 + position.x).max(0)
          as u32,
        y: previous_option_bottom + option_spacing,
      };

      renderer.render_image(&position, image_asset, &RENDERED_WINDOW_DIMENSIONS)?;

      previous_option_bottom = position.y + image_height;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod menu_option_data_trait_tests {
    use super::test_data::*;
    use super::*;

    #[test]
    fn defaults_return_expected_values() {
      let default_options = DefaultMenu::First;

      let expected_name = "unknown";

      assert_eq!(default_options.option_name(), expected_name);
      assert_eq!(default_options.asset_name(), expected_name);
    }

    #[test]
    fn non_default_returns_expected() {
      let options = TestMenu::Start;

      let expected_option_name = "start";
      let expected_asset_name = "start_asset";

      assert_eq!(options.option_name(), expected_option_name);
      assert_eq!(options.asset_name(), expected_asset_name);
    }

    #[test]
    fn cursor_moves_as_expected() {
      let options = vec![TestMenu::Start, TestMenu::Options, TestMenu::Exit];
      let mut menu = Menu::new("test_menu", options.clone());

      let expected_options: Vec<MenuOption> = options.into_iter().map(MenuOption::from).collect();

      menu.next();
      assert_eq!(menu.current_option(), expected_options.get(1));
      menu.next();
      assert_eq!(menu.current_option(), expected_options.get(2));
      // Wrap back to 0
      menu.next();
      assert_eq!(menu.current_option(), expected_options.get(0));

      // Wrap back to last item in the list.
      menu.previous();
      assert_eq!(menu.current_option(), expected_options.get(2));
    }

    #[test]
    fn empty_list_returns_no_item() {
      let options: Vec<TestMenu> = vec![];
      let mut menu = Menu::new("test_menu", options);

      assert!(menu.current_option().is_none());

      menu.next();
      assert!(menu.selected == 0);

      menu.previous();
      assert!(menu.selected == 0);
    }
  }

  mod test_data {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestMenu {
      Start,
      Options,
      Exit,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DefaultMenu {
      First,
    }

    impl MenuOptionData for TestMenu {
      fn asset_name(&self) -> &'static str {
        match self {
          TestMenu::Start => "start_asset",
          TestMenu::Options => "options_asset",
          TestMenu::Exit => "exit_asset",
        }
      }

      fn option_name(&self) -> &'static str {
        match self {
          TestMenu::Start => "start",
          TestMenu::Options => "options",
          TestMenu::Exit => "exit",
        }
      }
    }

    impl From<TestMenu> for MenuOption {
      fn from(item: TestMenu) -> MenuOption {
        MenuOption::new(item)
      }
    }

    impl From<DefaultMenu> for MenuOption {
      fn from(item: DefaultMenu) -> MenuOption {
        MenuOption::new(item)
      }
    }

    impl MenuOptionData for DefaultMenu {}
  }
}
