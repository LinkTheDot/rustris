use crate::asset_loader::*;
use crate::game::world_data::WorldData;
use crate::get_renderable_from_name;
pub use crate::menus::menu_items::*;
use crate::renderer::renderable::Renderable;
use crate::renderer::*;
use anyhow::anyhow;
use winit::dpi::*;

/// Creating a menu is best done through the [`define_menu_items`](crate::define_menu_items) macro.
/// This macro will easily define every item in a menu, and its corresponding asset.menu
///
/// ```
/// use rustris::define_menu_items;
/// use rustris::menus::menu_data::*;
///
/// define_menu_items! {
///   pub const POSITION = LogicalPosition { x: 20, y: 20 };
///   pub const ITEM_OFFSET = 10;
///
///   pub enum MyMenu {
///     ItemOne(item_name = "item_one", asset_name = "item_one_menu_image"),
///   }
/// }
///
/// // The actual code for this is commented as the assets do not exist in this example.
///
/// // let menu = Menu::new::<MyMenu>().unwrap();
///
/// // let current_option = menu.current_option().unwrap();
/// // let current_option = MyMenu::from_menu_item(current_option);
///
/// // assert_eq!(current_option, Some(MyMenu::ItemOne));
/// // assert_eq!(MyMenu::MENU_NAME, "MyMenu");
/// ```
#[derive(Debug, Clone)]
pub struct Menu {
  name: &'static str,
  /// The index for which option is currently selected.
  selected: usize,
  /// The list of menu items, containing their name and the name of their corresponding asset.
  menu_items: Vec<MenuItem>,
  /// The positions of each item in order calculated upon creation of the menu.
  menu_item_positions: Vec<LogicalPosition<u32>>,
}

impl Menu {
  /// Creates a new menu from a list of options.
  ///
  /// Each option will implement [`MenuItemData`](crate::menus::menu_items::MenuItemData).
  /// This will force each option to know its name, and what asset it's tied to.
  /// This allows for better organization of the possible options in a menu.
  ///
  /// # Example
  ///
  /// ```
  /// use rustris::menus::menu_data::Menu;
  /// use rustris::define_menu_items;
  ///
  /// define_menu_items! {
  ///   pub const POSITION = LogicalPosition { x: 20, y: 20 };
  ///   pub const ITEM_OFFSET = 10;
  ///
  ///   pub enum MyMenu {
  ///     ItemOne(item_name = "my_menu_item_one", asset_name = "my_menu_item_one_button_asset"),
  ///   }
  /// }
  ///
  /// // The actual code for this is commented as the assets do not exist in this example.
  ///
  /// // let menu = Menu::new::<MyMenu>().unwrap();
  /// ```
  ///
  ///
  /// # Errors
  ///
  /// - The menu contains asset names that don't exist in the [`asset_loader`](crate::asset_loader).
  pub fn new<Menu: MenuItemData>() -> anyhow::Result<Self> {
    let menu_items = Menu::full_list();
    let name = Menu::MENU_NAME;
    let menu_item_positions = Vec::with_capacity(menu_items.len());

    let mut menu = Self {
      name,
      selected: 0,
      menu_items,
      menu_item_positions,
    };

    menu.calculate_item_positions(Menu::POSITION, Menu::ITEM_OFFSET)?;

    Ok(menu)
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
    let option_count = self.menu_items.len();

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
    let option_count = self.menu_items.len();

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
    if self.menu_items.is_empty() {
      return None;
    }

    let selected_option = self.menu_items.get(self.selected);

    if selected_option.is_none() {
      log::error!(
        "Attempted to index a menu option that doesn't exist. Index: {}, Max: {}",
        self.selected,
        self.menu_items.len()
      );

      return self.menu_items.first();
    }

    selected_option
  }

  /// Renders all menu items based on their assigned positions on creation of the menu.
  pub fn render(&self, renderer: &mut Renderer, color: Option<[u8; 4]>) -> anyhow::Result<()> {
    if self.menu_items.is_empty() {
      log::warn!("Attempted to render a menu with no items.");

      return Ok(());
    }

    // Sanity check
    if self.menu_item_positions.is_empty() {
      return Err(anyhow!(
        "Attempted to render a menu with an incorrectly sized list of menu item positions."
      ));
    }

    // Sanity check
    if self.menu_items.len() != self.menu_item_positions.len() {
      return Err(anyhow!(
        "The amount of menu items does not match the amount of stored menu item positions."
      ));
    }

    let color = color.unwrap_or([0; 4]);

    self
      .menu_items
      .iter()
      .enumerate()
      .try_for_each(|(index, menu_item)| {
        let position = self.menu_item_positions.get(index).unwrap();
        let asset_name = menu_item.asset_name();

        if let Some(renderable_asset) = get_renderable_from_name(asset_name) {
          renderable_asset.render(renderer, position, &color)
        } else {
          Err(anyhow!(
            "Attempted to render a menu with a missing asset of name {:?}",
            asset_name
          ))
        }
      })
  }

  /// This is called upon creation of the menu based on the settings defined in menu creation.
  ///
  /// If updating the position is desired, use this method.
  ///
  /// When rendering a mix of images and text boxes, text boxes will ignore the offset and position of images.
  ///
  /// The textboxes are still acounted for when it comes to the offset of images.
  /// That means if you have an incorrectly positioned text box, the rendered images surrounding that textbox
  /// will be placed further apart than expected due to the option_spacing.
  ///
  /// # Errors
  ///
  /// - The menu contains asset names that don't exist in the [`asset_loader`](crate::asset_loader).
  pub fn calculate_item_positions(
    &mut self,
    starting_position: LogicalPosition<u32>,
    item_offset: u32,
  ) -> anyhow::Result<()> {
    self.menu_item_positions.clear();

    let mut offset = 0;

    self
      .menu_items
      .iter()
      .map(MenuItem::asset_name)
      .try_for_each(|asset_name| {
        if let Some(image) = Assets::get_image(asset_name) {
          let LogicalSize {
            width: image_width,
            height: image_height,
          } = image.dimensions();

          let position = LogicalPosition {
            x: starting_position.x - (image_width / 2),
            y: starting_position.y + offset,
          };

          self.menu_item_positions.push(position);

          offset += image_height;
        } else {
          let Some(text_box) = WorldData::get_text_boxes().get(asset_name) else {
            return Err(anyhow!(
              "Failed to get asset {:?}, which doesn't exist.",
              asset_name
            ));
          };

          self.menu_item_positions.push(text_box.position());

          offset += text_box.dimensions().height;
        }

        offset += item_offset;

        Ok(())
      })
  }

  /// Returns the asset name of the asset at the given index.
  ///
  /// None is returned if the index is larger than the available items.
  pub fn asset_name_at_index(&self, index: usize) -> Option<&'static str> {
    self.menu_items.get(index).map(MenuItem::asset_name)
  }

  /// Returns the top left position of the asset at the given index.
  ///
  /// None is returned if the index is larger than the available items.
  pub fn asset_position_at_index(&self, index: usize) -> Option<&LogicalPosition<u32>> {
    self.menu_item_positions.get(index)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::menus::templates::main_menu::MainMenu;

  #[test]
  fn cursor_moves_as_expected() {
    let mut menu = Menu::new::<MainMenu>().unwrap();
    let expected_options: Vec<MenuItem> = MainMenu::full_list();

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
}
