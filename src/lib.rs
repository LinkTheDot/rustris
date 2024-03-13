pub mod general_data {
  pub mod logging;
  pub mod result_traits;
  pub mod winit_traits;
}

pub mod game {
  pub mod actions;
  pub mod game_settings;
  pub mod minos;
  pub mod timer;
  pub mod world_data;
  pub mod world_state;
}

pub mod menus {
  pub mod templates {
    pub mod game_settings;
    pub mod main_menu;
  }

  pub mod menu_data;
  pub mod menu_items;
}

pub mod renderer;

pub mod asset_loader;
pub mod rustris_config;

/// Obtains an asset that implements [`Renderable`](crate::renderable::Renderable) from its name.
///
/// None is returned if no asset by the given name exists.
pub fn get_renderable_from_name(
  asset_name: &'static str,
) -> Option<&dyn crate::renderer::renderable::Renderable> {
  use crate::asset_loader::*;
  use crate::game::world_data::WorldData;
  use crate::renderer::renderable::Renderable;

  match AssetType::from_name(asset_name) {
    Some(AssetType::Image) => Assets::get_image(asset_name).map(|image| image as &dyn Renderable),
    _ => WorldData::get_text_boxes()
      .get(asset_name)
      .map(|text_box| text_box as &dyn Renderable),
  }
}
