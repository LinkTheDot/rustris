use image::DynamicImage;
use maplit::*;
use std::collections::HashMap;

/// Stores the bytes of the given path into the binary at compile time.
///
/// On run time, calls [`image::load_from_memory`](https://docs.rs/image/0.24.9/image/fn.load_from_memory.html) with the stored binary.
///
/// # Errors
/// - When [`image::load_from_memory`](https://docs.rs/image/0.24.9/image/fn.load_from_memory.html) returns an error.
macro_rules! image_from_path {
  ($path:literal) => {
    match image::load_from_memory(include_bytes!(concat!(env!("PWD"), $path))) {
      Ok(image) => image,
      Err(error) => {
        log::error!("Failed to load image at path {:?}", $path);

        panic!("{:?}", error);
      }
    }
  };
}

pub struct Assets {
  image_assets: HashMap<&'static str, DynamicImage>,
  font_assets: HashMap<&'static str, &'static [u8]>,
}

impl Assets {
  pub fn load_assets() -> Self {
    let image_assets = Self::load_image_assets();
    let font_assets = Self::load_font_assets();

    Self {
      image_assets,
      font_assets,
    }
  }

  pub fn get_image(&self, image_name: &'static str) -> Option<&DynamicImage> {
    self.image_assets.get(image_name)
  }

  pub fn get_font(&self, font_name: &'static str) -> Option<&&'static [u8]> {
    self.font_assets.get(font_name)
  }

  pub fn image_assets(&self) -> &HashMap<&'static str, DynamicImage> {
    &self.image_assets
  }

  pub fn font_assets(&self) -> &HashMap<&'static str, &'static [u8]> {
    &self.font_assets
  }

  fn load_image_assets() -> HashMap<&'static str, DynamicImage> {
    hashmap! {
      "menu_start_v1" => image_from_path!("/assets/start_v1.png"),
      "menu_start_v2" => image_from_path!("/assets/start_v2.png"),
      "menu_options" => image_from_path!("/assets/options.png"),
      "menu_exit" => image_from_path!("/assets/exit.png"),
      "menu_background" => image_from_path!("/assets/background.png"),
    }
  }

  fn load_font_assets() -> HashMap<&'static str, &'static [u8]> {
    hashmap! {
      "gadugi" => include_bytes!(concat!(env!("PWD"), "/assets/gadugi-normal.ttf")) as &[u8],
    }
  }
}
