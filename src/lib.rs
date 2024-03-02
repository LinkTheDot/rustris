pub mod general_data {
  pub mod logging;
  pub mod result_traits;
  pub mod winit_traits;
}

pub mod game {
  pub mod actions;
  pub mod game_settings;
  pub mod minos;
  pub mod world_data;
  pub mod world_state;
}

pub mod menus {
  pub mod templates {
    pub mod game_settings;
    pub mod main_menu;
  }

  pub mod menu_data;
  pub mod menu_option;
}

pub mod renderer;

pub mod asset_loader;
pub mod rustris_config;
