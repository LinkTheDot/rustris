pub use crate::menus::menu_items::*;
use crate::renderer::*;
use crate::{asset_loader::Assets, rustris_config::RENDERED_WINDOW_DIMENSIONS};
use anyhow::anyhow;
use image::GenericImageView;
use winit::dpi::*;

/// Creating a menu is best done through the [`define_menu_items`](crate::define_menu_items) macro.
/// This macro will easily define every item in a menu, and its corresponding asset.
///
/// ```
/// use rustris::define_menu_items;
/// use rustris::menus::menu_data::*;
///
/// define_menu_items! {
///   pub enum MyMenu {
///     ItemOne(item_name = "item_one", asset_name = "item_one_menu_image"),
///   }
/// }
///
/// let menu_name = "my_menu";
/// let menu = Menu::new::<MyMenu>(menu_name);
///
/// let current_option = menu.current_option().unwrap();
/// let current_option = MyMenu::from_menu_item(current_option);
///
/// assert_eq!(current_option, Some(MyMenu::ItemOne))
/// ```
#[derive(Debug, Clone)]
pub struct Menu {
  name: &'static str,
  /// The index for which option is currently selected.
  selected: usize,
  options: Vec<MenuItem>,
}

impl Menu {
  /// Creates a new menu from a list of options.
  ///
  /// Each option will implement [`MenuItemData`](crate::menus::menu_items::MenuItemData).
  /// This will force each option to know its name, and what asset it's tied to.
  /// This allows for better organization of the possible options in a menu.
  pub fn new<M: MenuItemData>(name: &'static str) -> Self {
    let options = M::full_list();

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
  pub fn current_option(&self) -> Option<&MenuItem> {
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

      return self.options.first();
    }

    selected_option
  }

  /// Renders the menu to the buffer with the given offset and option spacing.
  ///
  /// The option_spacing is the gap between each option in pixels, not the space between the center of each image.
  pub fn render(
    &self,
    assets: &Assets,
    position: &LogicalPosition<i32>,
    renderer: &mut Renderer,
    option_spacing: u32,
  ) -> anyhow::Result<()> {
    let mut previous_option_bottom = position.y as u32;

    for menu_option in self.options.iter() {
      let Some(image_asset) = assets.get_image(menu_option.asset_name()) else {
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
  use test_data::*;

  #[test]
  fn cursor_moves_as_expected() {
    let mut menu = Menu::new::<TestMenu>("test_menu");

    let expected_options: Vec<MenuItem> = TestMenu::full_list();

    menu.next();
    assert_eq!(menu.current_option(), expected_options.get(1));
    menu.next();
    assert_eq!(menu.current_option(), expected_options.get(2));
    // Wrap back to 0
    menu.next();
    assert_eq!(menu.current_option(), expected_options.first());

    // Wrap back to last item in the list.
    menu.previous();
    assert_eq!(menu.current_option(), expected_options.get(2));
  }

  mod test_data {
    use super::*;
    use crate::define_menu_items;

    define_menu_items! {
      pub enum TestMenu {
        Start(item_name = "start", asset_name = "start_asset"),
        Options(item_name = "options", asset_name = "options_asset"),
        Exit(item_name = "exit", asset_name = "exit_asset"),
      }
    }
  }
}
