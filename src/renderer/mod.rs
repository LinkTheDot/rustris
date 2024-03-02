#![forbid(unsafe_code)]

use anyhow::anyhow;
use fontdue::Font;
use image::DynamicImage;
use pixels::Pixels;
use winit::dpi::*;

use self::fonts::TextBox;

pub mod fonts;

pub struct Renderer {
  pixels: Pixels,

  loaded_fonts: Vec<Font>,
  font_layout_by_name: Vec<&'static str>,
}

impl Renderer {
  pub fn new(pixels: Pixels) -> Self {
    Self {
      pixels,
      loaded_fonts: Vec::with_capacity(2),
      font_layout_by_name: Vec::with_capacity(2),
    }
  }

  /// Calls `.render()` on the contained pixels::Pixels.
  pub fn complete_render(&self) -> anyhow::Result<()> {
    self.pixels.render().map_err(Into::into)
  }

  /// Resizes the internal surface.
  pub fn resize_surface(&mut self, new_dimensions: PhysicalSize<u32>) -> anyhow::Result<()> {
    self
      .pixels
      .resize_surface(new_dimensions.width.max(1), new_dimensions.height.max(1))
      .map_err(Into::into)
  }

  /// Replaces every pixel in the buffer with the given color.
  pub fn set_color(&mut self, rgb: [u8; 3]) -> anyhow::Result<()> {
    for (iteration, byte) in self.pixels.frame_mut().iter_mut().enumerate() {
      *byte = match iteration % 4 {
        3 => 255,
        n => rgb[2 - n],
      };
    }

    Ok(())
  }

  pub fn clear(&mut self) -> anyhow::Result<()> {
    for (iteration, byte) in self.pixels.frame_mut().iter_mut().enumerate() {
      *byte = if iteration % 4 == 3 { 255 } else { 0 };
    }

    Ok(())
  }

  /// Applies the color with the given alpha to every pixel on the screen.
  pub fn apply_color(&mut self, rgba: [u8; 4]) -> anyhow::Result<()> {
    let buffer = self.pixels.frame_mut();
    let pixel_count = buffer.len() / 4;

    for index in 0..pixel_count {
      Self::draw_at_pixel_with_rgba(buffer, index, &rgba)?;
    }

    Ok(())
  }

  /// Returns a mutable reference to the frame buffer.
  pub fn frame_mut(&mut self) -> &mut [u8] {
    self.pixels.frame_mut()
  }

  /// Returns a reference to the frame buffer.
  pub fn frame(&self) -> &[u8] {
    self.pixels.frame()
  }

  pub fn draw_rectangle(
    &mut self,
    position: &LogicalPosition<u32>,
    dimensions: &LogicalSize<u32>,
    color: [u8; 4],
    buffer_dimensions: &LogicalSize<u32>,
  ) -> anyhow::Result<()> {
    let buffer = self.pixels.frame_mut();

    let LogicalSize {
      width: rectangle_width,
      height: rectangle_height,
    } = dimensions;

    let top_left_placement = position.x + (position.y * buffer_dimensions.width);

    for index in 0..(rectangle_width * rectangle_height) {
      let window_index = top_left_placement
        + (index % rectangle_width)
        + ((index / rectangle_width) * buffer_dimensions.width);

      Self::draw_at_pixel_with_rgba(buffer, window_index as usize, &color)?;
    }

    Ok(())
  }

  pub fn render_image(
    &mut self,
    offset: &LogicalPosition<u32>,
    image: &DynamicImage,
    window_dimensions: &LogicalSize<u32>,
  ) -> anyhow::Result<()> {
    let image_width = image.width();
    let image_height = image.height();

    let Some(image_buffer) = image.as_rgba8() else {
      return Err(anyhow!("Failed to read image as rgba8 when rendering."));
    };

    let frame_buffer = self.pixels.frame_mut();
    let position = offset;
    let top_left = position.x + (position.y * window_dimensions.width);
    let image_buffer = image_buffer.chunks_exact(4);

    for (index, rgba) in (0..(image_width * image_height)).zip(image_buffer) {
      let rgba: &[u8; 4] = rgba.try_into()?;
      let (x, y) = (index % image_width, index / image_width);
      let buffer_index = (top_left + x + (y * window_dimensions.width)) as usize;

      Self::draw_at_pixel_with_rgba(frame_buffer, buffer_index, rgba)?
    }

    Ok(())
  }

  /// Loads a font into memory from a font file's bytes.
  ///
  /// Stored in a list, [`render_font()`](Renderer::render_font) uses the index of these stored fonts.
  /// The index is in the order by which the fonts were loaded.
  pub fn load_font_from_bytes(
    &mut self,
    font_data: &[u8],
    font_name: &'static str,
  ) -> anyhow::Result<()> {
    let font = match Font::from_bytes(font_data, fontdue::FontSettings::default()) {
      Ok(font) => font,
      Err(error) => return Err(anyhow!("Failed to load the font: `{:?}`", error)),
    };

    self.loaded_fonts.push(font);
    self.font_layout_by_name.push(font_name);

    Ok(())
  }

  /// Renders the text for the given [`TextBox`](crate::renderer::fonts::TextBox).
  pub fn render_text_box(
    &mut self,
    text_box: &TextBox,
    color: [u8; 4],
    buffer_dimensions: &LogicalSize<u32>,
  ) -> anyhow::Result<()> {
    let Some(font_index) = text_box.font_index() else {
      log::warn!("Attempted to render an empty text box.");

      return Ok(());
    };
    let position = text_box.position().unwrap();

    let Some(font) = self.loaded_fonts.get(font_index) else {
      return Err(anyhow!(
        "Attempted to load a font that didn't exist. Index: {}, Font count: {}",
        font_index,
        self.loaded_fonts.len()
      ));
    };

    let buffer = self.pixels.frame_mut();
    let top_left_placement = position.x + (position.y * buffer_dimensions.width);

    let result: anyhow::Result<()> = text_box.character_data().iter().try_for_each(|glyph| {
      log::debug!("Rendering {:?}", glyph.parent);

      if !glyph.parent.is_ascii() {
        return Err(anyhow!(
          "Attempted to render a non-ascii character: `{:?}`",
          glyph.parent
        ));
      }

      let (metadata, bitmap) = font.rasterize(glyph.parent, glyph.key.px);
      let (text_width, text_height) = (glyph.width as u32, metadata.height as u32);

      log::debug!("dimensions: {}x{}", text_width, text_height);
      log::debug!("Metadata: {:#?}", metadata);

      for index in 0..(text_width * text_height) {
        let position = top_left_placement
          + glyph.x.cast::<u32>()
          + (glyph.y.cast::<u32>() * buffer_dimensions.width);

        let shade_percentage = (bitmap[index as usize] as u16 * 100) / 255;

        if shade_percentage == 0 {
          continue;
        }

        let color = [
          ((color[0] as u16 * shade_percentage) / 100).min(255) as u8,
          ((color[1] as u16 * shade_percentage) / 100).min(255) as u8,
          ((color[2] as u16 * shade_percentage) / 100).min(255) as u8,
          color[3],
        ];

        Self::draw_at_pixel_with_rgba(buffer, position as usize, &color)?;
      }

      Ok(())
    });

    if let Err(error) = result {
      return Err(anyhow!("Failed to render the text. `{:?}`", error));
    }

    Ok(())
  }

  pub fn fonts(&self) -> &Vec<Font> {
    &self.loaded_fonts
  }

  pub fn fonts_with_names(&self) -> Vec<(&'static str, &Font)> {
    self
      .font_layout_by_name
      .iter()
      .zip(self.loaded_fonts.iter())
      .map(|(font_name, font)| (*font_name, font))
      .collect()
  }

  /// Draws at the pixel in the frame buffer.
  ///
  /// This method allows for easier calculating for the index into this buffer.
  /// The frame buffer is an array of u8, where every chunk of 4 is an actual pixel.
  /// The index passed in will point to the actual pixel desired.
  ///
  /// The alpha channel is turned into a percentage value from 0-100. The lower this value the more transparent
  /// the given rgb value is when blending.
  #[inline]
  pub fn draw_at_pixel_with_rgba(
    pixel_buffer: &mut [u8],
    pixel_index: usize,
    rgba: &[u8; 4],
  ) -> anyhow::Result<()> {
    // Alpha is 0, meaning this rgb value is completely transparent.
    if rgba[3] == 0 {
      return Ok(());
    }

    let adjusted_pixel_index = pixel_index * 4;
    let pixel_buffer_length = pixel_buffer.len();

    if pixel_buffer_length < adjusted_pixel_index + 4 {
      return Err(anyhow!(
        "Attempted to index out of bounds of the pixel buffer. buffer_length: {}, max_index: {}",
        pixel_buffer_length,
        adjusted_pixel_index + 4
      ));
    }

    // Get the first 3 bytes of the pixel, as the last bytes if the alpha channel.
    let pixel_color = &mut pixel_buffer[(adjusted_pixel_index)..(adjusted_pixel_index + 3)];

    if rgba[3] == 255 {
      pixel_color.copy_from_slice(&rgba[0..3]);

      return Ok(());
    }

    // A range between 0-100 to determine the percentage in the alpha channel.
    // The higher the alpha the less transparent the pixel.
    let alpha_percentage = 100 - (rgba[3] as u16 * 100) / 255;

    // Prevents having to cast every pixel into f32, instead casting into a smaller u16.
    // BlendedColor = ((alpha_percent * top_color) / 100) + ((alpha_percent * bottom_color) / 100)
    for index in 0..3 {
      let top_color = rgba[index] as u16;
      let bottom_color = pixel_color[index] as u16;

      pixel_color[index] =
        (((alpha_percentage * top_color) / 100) + ((alpha_percentage * bottom_color) / 100)) as u8;
    }

    Ok(())
  }

  /// Draws at the pixel in the frame buffer.
  ///
  /// This method allows for easier calculating for the index into this buffer.
  /// The frame buffer is an array of u8, where every chunk of 4 is an actual pixel.
  /// The index passed in will point to the actual pixel desired.
  #[inline]
  pub fn draw_at_pixel_with_rgb(
    pixel_buffer: &mut [u8],
    pixel_index: usize,
    rgb: &[u8; 3],
  ) -> anyhow::Result<()> {
    let adjusted_pixel_index = pixel_index * 4;
    let pixel_buffer_length = pixel_buffer.len();

    if pixel_buffer_length < adjusted_pixel_index + 4 {
      return Err(anyhow!(
        "Attempted to index out of bounds of the pixel buffer. buffer_length: {}, max_index: {}",
        pixel_buffer_length,
        adjusted_pixel_index + 4
      ));
    }

    // Get the first 3 bytes of the pixel, as the last bytes if the alpha channel.
    let pixel_color = &mut pixel_buffer[(adjusted_pixel_index)..(adjusted_pixel_index + 3)];

    pixel_color.copy_from_slice(rgb);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod draw_at_pixel_logic {
    use super::*;

    #[test]
    fn rgb_applies_correct_alpha_channel() {
      let mut pixel_buffer = [0x77, 0x77, 0x77, 0xFF];
      let rgb = [0xFF, 0xFF, 0xFF];

      let expected_pixel_buffer = [0xFF, 0xFF, 0xFF, 0xFF];

      Renderer::draw_at_pixel_with_rgb(&mut pixel_buffer, 0, &rgb).unwrap();

      assert_eq!(pixel_buffer, expected_pixel_buffer);
    }

    #[test]
    fn modifies_correct_index() {
      let mut pixel_buffer = [
        0xFF, 0xFF, 0xFF, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF,
      ];
      let replacement_color = [0x77, 0x77, 0x77, 0xFF];

      let expected_pixel_buffer = [
        0x77, 0x77, 0x77, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF, //
        0x77, 0x77, 0x77, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF,
      ];

      Renderer::draw_at_pixel_with_rgba(&mut pixel_buffer, 0, &replacement_color).unwrap();
      Renderer::draw_at_pixel_with_rgba(&mut pixel_buffer, 2, &replacement_color).unwrap();

      assert_eq!(pixel_buffer, expected_pixel_buffer);
    }

    #[test]
    fn alpha_blending_works_when_drawing() {
      let mut pixel_buffer = [0x77, 0x77, 0x77, 0xFF];
      let blending_rgba = [0xFF, 0xFF, 0xFF, 0x7F];

      // BlendedColor = ((alpha_percent * top_color) / 100) + ((alpha_percent * bottom_color) / 100)
      let alpha_percentage = 100 - (blending_rgba[3] as u16 * 100) / 255;
      let top_color = blending_rgba[0] as u16;
      let bottom_color = pixel_buffer[1] as u16;
      let expected_color =
        (((alpha_percentage * top_color) / 100) + ((alpha_percentage * bottom_color) / 100)) as u8;

      Renderer::draw_at_pixel_with_rgba(&mut pixel_buffer, 0, &blending_rgba).unwrap();

      assert_eq!(
        pixel_buffer,
        [expected_color, expected_color, expected_color, 0xFF]
      );
    }

    #[test]
    fn full_alpha_replaces_entire_color() {
      let mut pixel_buffer = [0x77, 0x77, 0x77, 0xFF];
      let rgba = [0xFF, 0xFF, 0xFF, 0xFF];

      let expected_color = [0xFF, 0xFF, 0xFF, 0xFF];

      Renderer::draw_at_pixel_with_rgba(&mut pixel_buffer, 0, &rgba).unwrap();

      assert_eq!(pixel_buffer, expected_color);
    }

    #[test]
    fn zeroed_alpha_channel_does_nothing_to_color() {
      let mut pixel_buffer = [0x77, 0x77, 0x77, 0xFF];
      let rgba = [0xFF, 0xFF, 0xFF, 0];

      let expected_color = pixel_buffer;

      Renderer::draw_at_pixel_with_rgba(&mut pixel_buffer, 0, &rgba).unwrap();

      assert_eq!(pixel_buffer, expected_color);
    }
  }
}
