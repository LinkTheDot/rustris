use anyhow::anyhow;
use image::imageops::FilterType;
use image::DynamicImage;
use pixels::Pixels;
use winit::dpi::Pixel;
use winit::dpi::*;

pub struct Renderer {
  pixels: Pixels,
}

impl Renderer {
  pub fn new(pixels: Pixels) -> Self {
    Self { pixels }
  }

  /// Calls `.render()` on the contained pixels::Pixels.
  pub fn complete_render(&self) -> anyhow::Result<()> {
    self.pixels.render().map_err(Into::into)
  }

  /// Replaces every pixel in the buffer with the given color.
  pub fn set_color(&mut self, rgb: [u8; 3]) {
    for pixel in self.pixels.frame_mut().chunks_exact_mut(4) {
      pixel.copy_from_slice(&[0, rgb[0], rgb[1], rgb[2]])
    }
  }

  pub fn render_image(
    &mut self,
    position: LogicalPosition<u32>,
    image: &DynamicImage,
    window_dimensions: &PhysicalSize<u32>,
    scale_factor: &f64,
  ) -> anyhow::Result<()> {
    // Nearest, Triangle, CatmullRom, Gaussian, Lanczos3
    const SCALE_TYPE: FilterType = FilterType::Gaussian;

    let mut image_width = image.width();
    let mut image_height = image.height();

    let scaled_image = if scale_factor != &1.0_f64 {
      image_width = (image_width as f64 * scale_factor).cast::<u32>();
      image_height = (image_height as f64 * scale_factor).cast::<u32>();

      Some(image.resize(
        (image_width as f64 * scale_factor).cast::<u32>(),
        (image_height as f64 * scale_factor).cast::<u32>(),
        SCALE_TYPE,
      ))
    } else {
      None
    };
    let Some(image_buffer) = scaled_image.as_ref().unwrap_or(image).as_rgba8() else {
      return Err(anyhow!("Failed to read image as rgba8 when rendering."));
    };

    let frame_buffer = self.pixels.frame_mut();
    let position = position.to_physical::<u32>(*scale_factor);
    let top_left = position.x + (position.y * window_dimensions.width);
    let image_buffer = image_buffer.chunks_exact(4);

    for (index, rgba) in (0..(image_width * image_height)).zip(image_buffer) {
      let rgba: &[u8; 4] = rgba.try_into()?;
      let (x, y) = (index % image_width, index / image_width);
      let buffer_index = (top_left + x + (y * window_dimensions.width)) as usize;

      Self::draw_at_pixel_literal_with_rgba(frame_buffer, buffer_index, rgba)?
    }

    Ok(())
  }

  /// Draws at the literal pixel in the frame buffer.
  ///
  /// The frame buffer is an array of u8, where every chunk of 4 is an actual pixel. This method allows for
  /// easier indexing into this buffer for modifying those actual pixels.
  ///
  /// The alpha channel is turned into a percentage value from 0-100. The lower this value the more transparent
  /// the given rgb value is when blending.
  #[inline]
  fn draw_at_pixel_literal_with_rgba(
    pixel_buffer: &mut [u8],
    literal_pixel_index: usize,
    rgba: &[u8; 4],
  ) -> anyhow::Result<()> {
    let adjusted_pixel_index = literal_pixel_index * 4;
    let pixel_buffer_length = pixel_buffer.len();

    if pixel_buffer_length < adjusted_pixel_index + 4 {
      return Err(anyhow!(
        "Attempted to index out of bounds of the pixel buffer. buffer_length: {}, max_index: {}",
        pixel_buffer_length,
        adjusted_pixel_index + 4
      ));
    }

    // Get the last 3 bytes of the pixel, as the first byte is useless data here.
    let pixel_color = &mut pixel_buffer[(adjusted_pixel_index + 1)..(adjusted_pixel_index + 4)];

    if rgba[3] == 255 {
      pixel_color.copy_from_slice(&rgba[0..3]);

      return Ok(());
    } else if rgba[3] == 0 {
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

  #[inline]
  fn draw_at_pixel_literal_with_rgb(
    pixel_buffer: &mut [u8],
    literal_pixel_index: usize,
    rgb: &[u8; 3],
  ) -> anyhow::Result<()> {
    let adjusted_pixel_index = literal_pixel_index * 4;
    let pixel_buffer_length = pixel_buffer.len();

    if pixel_buffer_length < adjusted_pixel_index + 4 {
      return Err(anyhow!(
        "Attempted to index out of bounds of the pixel buffer. buffer_length: {}, max_index: {}",
        pixel_buffer_length,
        adjusted_pixel_index + 4
      ));
    }

    // Get the last 3 bytes of the pixel, as the first byte is useless data here.
    let pixel_color = &mut pixel_buffer[(adjusted_pixel_index + 1)..(adjusted_pixel_index + 4)];

    pixel_color.copy_from_slice(rgb);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod draw_at_pixel_literal_logic {
    use super::*;

    #[test]
    fn literal_rgb_applies_correct_alpha_channel() {
      let mut pixel_buffer = [0, 0x77, 0x77, 0x77];
      let rgb = [0xFF, 0xFF, 0xFF];

      let expected_pixel_buffer = [0, 0xFF, 0xFF, 0xFF];

      Renderer::draw_at_pixel_literal_with_rgb(&mut pixel_buffer, 0, &rgb).unwrap();

      assert_eq!(pixel_buffer, expected_pixel_buffer);
    }

    #[test]
    fn modifies_correct_index() {
      let mut pixel_buffer = [
        0, 0xFF, 0xFF, 0xFF, //
        0, 0xFF, 0xFF, 0xFF, //
        0, 0xFF, 0xFF, 0xFF, //
        0, 0xFF, 0xFF, 0xFF,
      ];
      let replacement_color = [0x77, 0x77, 0x77, 0xFF];

      let expected_pixel_buffer = [
        0, 0x77, 0x77, 0x77, //
        0, 0xFF, 0xFF, 0xFF, //
        0, 0x77, 0x77, 0x77, //
        0, 0xFF, 0xFF, 0xFF,
      ];

      Renderer::draw_at_pixel_literal_with_rgba(&mut pixel_buffer, 0, &replacement_color).unwrap();
      println!("buffer inbetween: {:?}", pixel_buffer);
      Renderer::draw_at_pixel_literal_with_rgba(&mut pixel_buffer, 2, &replacement_color).unwrap();
      println!("buffer after: {:?}", pixel_buffer);

      assert_eq!(pixel_buffer, expected_pixel_buffer);
    }

    #[test]
    fn alpha_blending_works_when_drawing() {
      let mut pixel_buffer = [0, 0x77, 0x77, 0x77];
      let blending_rgba = [0xFF, 0xFF, 0xFF, 0x7F];

      // BlendedColor = ((alpha_percent * top_color) / 100) + ((alpha_percent * bottom_color) / 100)
      let alpha_percentage = 100 - (blending_rgba[3] as u16 * 100) / 255;
      let top_color = blending_rgba[0] as u16;
      let bottom_color = pixel_buffer[1] as u16;
      let expected_color =
        (((alpha_percentage * top_color) / 100) + ((alpha_percentage * bottom_color) / 100)) as u8;

      Renderer::draw_at_pixel_literal_with_rgba(&mut pixel_buffer, 0, &blending_rgba).unwrap();

      assert_eq!(
        pixel_buffer,
        [0, expected_color, expected_color, expected_color]
      );
    }

    #[test]
    fn full_alpha_replaces_entire_color() {
      let mut pixel_buffer = [0, 0x77, 0x77, 0x77];
      let rgba = [0xFF, 0xFF, 0xFF, 0xFF];

      let expected_color = [0, 0xFF, 0xFF, 0xFF];

      Renderer::draw_at_pixel_literal_with_rgba(&mut pixel_buffer, 0, &rgba).unwrap();

      assert_eq!(pixel_buffer, expected_color);
    }

    #[test]
    fn zeroed_alpha_channel_does_nothing_to_color() {
      let mut pixel_buffer = [0, 0x77, 0x77, 0x77];
      let rgba = [0xFF, 0xFF, 0xFF, 0x00];

      let expected_color = pixel_buffer;

      Renderer::draw_at_pixel_literal_with_rgba(&mut pixel_buffer, 0, &rgba).unwrap();

      assert_eq!(pixel_buffer, expected_color);
    }
  }
}
