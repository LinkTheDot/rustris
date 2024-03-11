#![forbid(unsafe_code)]

use anyhow::anyhow;
use pixels::Pixels;
use winit::dpi::*;

pub mod images;
pub mod renderable;
pub mod text_boxes;

pub struct Renderer {
  pixels: Pixels,
  buffer_dimensions: LogicalSize<u32>,
}

impl Renderer {
  /// # Errors
  ///
  /// - When the passed in buffer_dimensions does not match the actual size of the pixels' buffer
  pub fn new(pixels: Pixels, buffer_dimensions: LogicalSize<u32>) -> anyhow::Result<Self> {
    log::info!("Creating renderer.");

    if (pixels.frame().len() / 4)
      != buffer_dimensions.width as usize * buffer_dimensions.height as usize
    {
      return Err(anyhow!(
        "Frame buffer size does not match passed in buffer_dimensions: `{:?}`. Buffer size: {:?}",
        buffer_dimensions,
        pixels.frame().len()
      ));
    }

    Ok(Self {
      pixels,
      buffer_dimensions,
    })
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

  pub fn filled_rectangle(
    &mut self,
    position: &LogicalPosition<u32>,
    dimensions: &LogicalSize<u32>,
    color: [u8; 4],
  ) -> anyhow::Result<()> {
    let buffer = self.pixels.frame_mut();

    let LogicalSize {
      width: rectangle_width,
      height: rectangle_height,
    } = dimensions;

    let top_left_placement = position.x + (position.y * self.buffer_dimensions.width);

    for index in 0..(rectangle_width * rectangle_height) {
      let window_index = top_left_placement
        + (index % rectangle_width)
        + ((index / rectangle_width) * self.buffer_dimensions.width);

      Self::draw_at_pixel_with_rgba(buffer, window_index as usize, &color)?;
    }

    Ok(())
  }

  /// Draws a line between the two given points with the Bresenham algorithm implemented by the [`Bresenham`](https://crates.io/crates/bresenham) crate.
  ///
  /// # Errors
  ///
  /// - If any line has an x or y < 0.
  /// - Anytime [`draw_at_pixel_with_rgba`](Renderer::draw_at_pixel_with_rgba) errors.
  pub fn line(
    &mut self,
    point_one: (isize, isize),
    point_two: (isize, isize),
    color: &[u8; 4],
  ) -> anyhow::Result<()> {
    let buffer_width = self.buffer_dimensions.width as isize;
    let pixel_buffer = self.frame_mut();

    if point_one.0 == 0 || point_one.1 == 0 || point_two.0 == 0 || point_two.1 == 0 {
      return Err(anyhow!(
        "Attempted to draw a line {:?} -> {:?} which contains a negative integer.",
        point_one,
        point_two
      ));
    }

    bresenham::Bresenham::new(point_one, point_two).try_for_each(|(x, y)| {
      let index = x + (y * buffer_width);

      // Sanity check.
      if index < 0 {
        return Err(anyhow!(
          "Attempted to draw a line index underflow. Index {} < 0",
          index
        ));
      }

      let index = index as usize;

      Renderer::draw_at_pixel_with_rgba(pixel_buffer, index, color)
    })
  }

  /// Draws the outline of a rectangle with the points given.
  pub fn bounding_rectangle(
    &mut self,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
    color: &[u8; 4],
  ) -> anyhow::Result<()> {
    let top_right = (bottom_right.0, top_left.1);
    let bottom_left = (top_left.0, bottom_right.1);

    self.line(top_left, (top_right.0 + 1, top_right.1), color)?;
    self.line(bottom_right, top_right, color)?;
    self.line(top_left, (bottom_left.0, bottom_left.1 + 1), color)?;
    self.line(bottom_right, bottom_left, color)?;

    Ok(())
  }

  /// Draws an arrow left or right with the position being the end of the arrow.
  /// The length will determine how far the wings of the arrow stretch out.
  pub fn draw_arrow(
    &mut self,
    end_position: &LogicalPosition<u32>,
    length: u32,
    point_right: bool,
    color: &[u8; 4],
  ) -> anyhow::Result<()> {
    let end_position = LogicalPosition {
      x: end_position.x as isize,
      y: end_position.y as isize,
    };
    let length = length as isize;

    let direction_sign = if point_right { -1 } else { 1 };

    let arrow_back = LogicalPosition {
      x: end_position.x + (length * direction_sign),
      ..end_position
    };

    let wingspan = length;
    let wing_y = end_position.y + (wingspan / 2);

    let wing_end_position = LogicalPosition {
      x: end_position.x + ((length * direction_sign) / 2),
      y: wing_y,
    };

    self.line(
      (end_position.x, end_position.y),
      (arrow_back.x, arrow_back.y),
      color,
    )?;
    self.line(
      (end_position.x, end_position.y),
      (wing_end_position.x, wing_end_position.y),
      color,
    )?;
    self.line(
      (end_position.x, end_position.y),
      (wing_end_position.x, wing_end_position.y - (wingspan)),
      color,
    )?;

    Ok(())
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
