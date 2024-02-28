use crate::menus::menu_data::MenuOptionData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuOption {
  name: &'static str,
  asset_name: &'static str,
}

impl MenuOption {
  pub fn new(item: impl MenuOptionData) -> Self {
    Self {
      name: item.option_name(),
      asset_name: item.asset_name(),
    }
  }

  #[inline]
  pub fn name(&self) -> &'static str {
    self.name
  }

  #[inline]
  pub fn asset_name(&self) -> &'static str {
    self.asset_name
  }
}
