#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuItem {
  item_name: &'static str,
  asset_name: &'static str,
}

impl MenuItem {
  pub fn new(item_name: &'static str, asset_name: &'static str) -> Self {
    Self {
      item_name,
      asset_name,
    }
  }

  pub fn item_name(&self) -> &'static str {
    self.item_name
  }

  pub fn asset_name(&self) -> &'static str {
    self.asset_name
  }
}

/// This trait will label the items for a menu.
///
/// Each item will have a way to convert into the name of its button asset.
pub trait MenuItemData {
  const MENU_NAME: &'static str;

  /// Gets the name of an individual menu item.
  fn item_name(&self) -> &'static str {
    "unknown"
  }

  /// Gets the name of the asset for an individual menu item.
  /// The list of assets can be found in the [`asset_loader`](crate::asset_loader) module
  fn asset_name(&self) -> &'static str {
    "unknown"
  }

  /// The full list of strings for every menu item's name.
  fn item_name_list() -> Vec<&'static str>;

  /// The full list of strings for every menu item's asset name.
  fn asset_name_list() -> Vec<&'static str>;

  /// Returns the list of every possible menu item in order, converted into [`MenuItem`](MenuItem)s
  fn full_list() -> Vec<MenuItem>;

  /// Converts an instance of [`MenuItem`](MenuItem) into Self.
  ///
  /// None is returned if the name of the MenuItem does not match any item_names under Self.
  fn from_menu_item(item: &MenuItem) -> Option<Self>
  where
    Self: Sized;
}

/// Defines the creation of an enum that can be used to create a [`Menu`](crate::menus::menu_data::Menu).
///
/// When defining a menu, each variant needs an item name and asset name.
/// The syntax for creating will look something like this:
///
/// ```
/// use rustris::define_menu_items;
/// use rustris::menus::menu_data::*;
///
/// define_menu_items! {
///   pub enum MainMenu {
///     Start(item_name = "start", asset_name = "menu_start"),
///     Settings(item_name = "settings", asset_name = "menu_settings"),
///     Exit(item_name = "exit", asset_name = "menu_exit"),
///   }
/// }
/// ```
///
/// This will expand into creating the enum and implementing [`MenuItemData`](MenuItemData) and Into<[`MenuItem`](MenuItem)>,
/// MenuItemData will allow for each item in the enum to have methods for obtaining the item's
/// name and the name of its corresponding asset.
/// Into<[`MenuItem`](MenuItem)> Will allow for the creation of a [`Menu`](crate::menus::menu_data::Menu).
///
/// Creating a menu will end up looking like this:
///
/// ```ignore,no_run
/// let main_menu = Menu::new::<MainMenu>();
/// ```
///
/// To uniquely store an instance of your menu items, you can use the following:
/// ```ignore,no_run
/// let mut menu_list: std::collections::HashMap<&'static str, Menu> = std::collections::HashMap::new();
///
/// menu_list.insert(MainMenu::MENU_NAME, main_menu);
/// ```
#[macro_export]
macro_rules! define_menu_items {
  {
    pub enum $name:ident {
      $($variant:ident ( item_name = $name_value:literal, asset_name = $asset_value:literal ) ),* $(,)?
    }
  } => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum $name {
      $($variant),*
    }

    impl $crate::menus::menu_items::MenuItemData for $name {
      const MENU_NAME: &'static str = stringify!($name);

      fn item_name(&self) -> &'static str {
        match &self {
          $(Self::$variant => $name_value),*,
        }
      }

      fn asset_name(&self) -> &'static str {
        match &self {
          $(Self::$variant => $asset_value),*,
        }
      }

      fn item_name_list() -> Vec<&'static str> {
        vec![
          $($name_value),*,
        ]
      }

      fn asset_name_list() -> Vec<&'static str> {
        vec![
          $($asset_value),*,
        ]
      }

      fn full_list() -> Vec<$crate::menus::menu_items::MenuItem> {
        vec![
          $($crate::menus::menu_items::MenuItem::from(&$name::$variant)),*,
        ]
      }

      fn from_menu_item(item: &$crate::menus::menu_items::MenuItem) -> Option<$name> {
        [
          $($name::$variant),*,
        ]
        .into_iter()
        .find(|menu_item| menu_item.item_name() == item.item_name())
      }
    }

    impl From<&$name> for $crate::menus::menu_items::MenuItem {
      fn from(menu_item: &$name) -> $crate::menus::menu_items::MenuItem {
        use $crate::menus::menu_items::MenuItemData;

        $crate::menus::menu_items::MenuItem::new(menu_item.item_name(), menu_item.asset_name())
      }
    }

    impl From<&$name> for &'static str {
      fn from(menu_item: &$name) -> &'static str {
        use $crate::menus::menu_items::MenuItemData;

        menu_item.item_name()
      }
    }

    impl From<$name> for &'static str {
      fn from(menu_item: $name) -> &'static str {
        use $crate::menus::menu_items::MenuItemData;
        menu_item.item_name()
      }
    }

  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_data::*;

  #[test]
  fn menu_item_returns_expected_items() {
    let options = TestMenu::Start;

    let expected_option_name = "start";
    let expected_asset_name = "start_asset";

    assert_eq!(options.item_name(), expected_option_name);
    assert_eq!(options.asset_name(), expected_asset_name);
  }

  #[test]
  fn list_returns_expected_items() {
    let list = TestMenu::full_list();

    let expected_list = vec![
      MenuItem::new("start", "start_asset"),
      MenuItem::new("options", "options_asset"),
      MenuItem::new("exit", "exit_asset"),
    ];

    assert_eq!(list, expected_list);
  }

  mod test_data {
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
