use crate::renderer::{renderable::Renderable, Renderer};
use anyhow::anyhow;
use image::DynamicImage;
use winit::dpi::LogicalPosition;

impl Renderable for DynamicImage {
  fn render(
    &self,
    renderer: &mut Renderer,
    position: &LogicalPosition<u32>,
    _color: &[u8; 4],
  ) -> anyhow::Result<()> {
    let image_width = self.width();
    let image_height = self.height();

    let Some(image_buffer) = self.as_rgba8() else {
      return Err(anyhow!("Failed to read image as rgba8 when rendering."));
    };

    let frame_buffer = renderer.pixels.frame_mut();
    let top_left = position.x + (position.y * renderer.buffer_dimensions.width);
    let image_buffer = image_buffer.chunks_exact(4);

    for (index, rgba) in (0..(image_width * image_height)).zip(image_buffer) {
      let rgba: &[u8; 4] = rgba.try_into()?;
      let (x, y) = (index % image_width, index / image_width);
      let buffer_index = (top_left + x + (y * renderer.buffer_dimensions.width)) as usize;

      Renderer::draw_at_pixel_with_rgba(frame_buffer, buffer_index, rgba)?
    }

    Ok(())
  }
}
