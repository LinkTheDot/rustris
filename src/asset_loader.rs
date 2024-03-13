//! Contains the [`Assets`](Assets) struct which acts as a namespace for accessing different assets.
//! Each asset type is initialized on first call using a [`OnceLock`](std::sync::OnceLock).
//!
//! ## Note
//!
//! Currently all assets of a given type are created on first call, which doesn't matter given the
//! scale of this project.

use fontdue::Font;
use image::DynamicImage;
use maplit::*;
use std::{collections::HashMap, sync::OnceLock};

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

/// The list of fonts to be initialized on first call.
static FONTS: OnceLock<Vec<Font>> = OnceLock::new();

/// This containts a map of font names and their correlated index in [`FONTS`](FONTS).
///
/// This is required because any method that fontdue uses for fonts requires a reference to an
/// array of Fonts, meaning it can't just be slapped in a hashmap. It must be &Vec<Font>, not Vec<&Font>.
static FONT_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();

/// The map of images to be initialized on first call.
static IMAGES: OnceLock<HashMap<&'static str, DynamicImage>> = OnceLock::new();

pub struct Assets;

pub enum AssetType {
  Image,
  Font,
}

impl AssetType {
  pub fn from_name(name: &'static str) -> Option<Self> {
    if Assets::get_image(name).is_some() {
      Some(Self::Image)
    } else if Assets::get_font(name).is_some() {
      Some(Self::Font)
    } else {
      None
    }
  }
}

impl Assets {
  /// Returns the full list of loaded fonts.
  pub fn get_font_list() -> &'static Vec<Font> {
    FONTS.get_or_init(Self::fonts)
  }

  pub fn get_font_names() -> &'static Vec<&'static str> {
    FONT_NAMES.get_or_init(Self::font_names)
  }

  /// Returns an individual font with the given name.
  ///
  /// None is returned if there was no font of that name.
  pub fn get_font(name: &'static str) -> Option<&'static Font> {
    let index = Self::get_font_names()
      .iter()
      .position(|font_name| font_name == &name)?;

    Self::get_font_list().get(index)
  }

  pub fn get_image_list() -> &'static HashMap<&'static str, DynamicImage> {
    IMAGES.get_or_init(Self::images)
  }

  pub fn get_image(image_name: &'static str) -> Option<&'static DynamicImage> {
    Self::get_image_list().get(image_name)
  }

  fn fonts() -> Vec<Font> {
    vec![Font::from_bytes(
      include_bytes!(concat!(env!("PWD"), "/assets/gadugi-normal.ttf")) as &[u8],
      fontdue::FontSettings::default(),
    )
    .unwrap()]
  }

  fn font_names() -> Vec<&'static str> {
    vec!["gadugi"]
  }

  fn images() -> HashMap<&'static str, DynamicImage> {
    hashmap! {
      "menu_start_v1" => image_from_path!("/assets/start_v1.png"),
      "menu_start_v2" => image_from_path!("/assets/start_v2.png"),
      "menu_options" => image_from_path!("/assets/options.png"),
      "menu_exit" => image_from_path!("/assets/exit.png"),
      "menu_background" => image_from_path!("/assets/background.png"),
    }
  }
}
