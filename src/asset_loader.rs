// use arcstr::ArcStr;
use image::DynamicImage;
use lazy_static::lazy_static;
use maplit::*;
use std::collections::HashMap;

/// Stores the bytes of the given path into the binary at compile time.
///
/// On run time, calls [`image::load_from_memory`](https://docs.rs/image/0.24.9/image/fn.load_from_memory.html) with the stored binary.
///
/// # Errors
/// - When [`image::load_from_memory`](https://docs.rs/image/0.24.9/image/fn.load_from_memory.html) returns an error.
macro_rules! image_from_path {
  ($literal:literal) => {
    match image::load_from_memory(include_bytes!(concat!(env!("PWD"), $literal))) {
      Ok(image) => image,
      Err(error) => {
        log::error!("Failed to load image at path {:?}", $literal);

        panic!("{:?}", error);
      }
    }
  };
}

lazy_static! {
  /// The list of all assets that will be loaded into the binary.
  pub static ref ASSETS: HashMap<&'static str, DynamicImage> = {
    hashmap! {
      "menu_start_v1" => image_from_path!("/assets/start-v1.png"),
      "menu_start_v2" => image_from_path!("/assets/start-v2.png"),
      "menu_options" => image_from_path!("/assets/options.png"),
      "menu_exit" => image_from_path!("/assets/exit.png"),
      "menu_background" => image_from_path!("/assets/background.png"),
    }
  };
}
