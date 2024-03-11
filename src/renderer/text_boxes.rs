use crate::asset_loader::Assets;
use crate::renderer::renderable::Renderable;
use crate::renderer::Renderer;
use anyhow::anyhow;
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle};
use winit::dpi::*;

pub struct TextBox {
  layout: Layout,
  dimensions: LogicalSize<u32>,
}

impl std::fmt::Debug for TextBox {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "TextBox {{ .. }}")
  }
}

impl TextBox {
  /// # Errors
  ///
  /// - If the font index is larger than the number of fonts.
  pub fn new(
    font_index: usize,
    text: &str,
    position: &LogicalPosition<u32>,
    size: f32,
  ) -> anyhow::Result<Self> {
    let font_list = Assets::get_font_list();

    if font_list.len() < font_index + 1 {
      return Err(anyhow!(
        "Attempted to create a TextBox with a font index out of bounds. Index: {} > {}",
        font_index,
        font_list.len()
      ));
    }

    let style = TextStyle::new(text, size, font_index);

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    let layout_settings = LayoutSettings {
      x: position.x as f32,
      y: position.y as f32,
      ..Default::default()
    };

    layout.reset(&layout_settings);
    layout.append(font_list, &style);

    let dimensions = Self::calculate_dimensions(layout.glyphs(), position);

    Ok(Self { layout, dimensions })
  }

  pub fn new_set_from_list(
    font_index: usize,
    font_size: f32,
    text_gap: u32,
    mut offset: LogicalPosition<u32>,
    list: Vec<(&'static str, &'static str)>,
  ) -> Vec<(&'static str, Self)> {
    list
      .into_iter()
      .filter_map(|(text_box_name, text)| {
        if text.is_empty() {
          offset.y += text_gap * 2;

          return None;
        }

        let text_box = match TextBox::new(font_index, text, &offset, font_size) {
          Ok(text_box) => text_box,
          Err(error) => {
            log::error!("Failed to create a text box from a list: {:?}", error);

            return None;
          }
        };

        let lowest_pixel = text_box
          .glyphs()
          .iter()
          .map(|glyph| glyph.y as u32 + glyph.height as u32)
          .min()
          .unwrap();

        offset.y = lowest_pixel + text_gap;

        Some((text_box_name, text_box))
      })
      .collect()
  }

  /// Updates the text contained in this textbox.
  ///
  /// If the textbox was originally an empty string, the font_index is set to 0.
  pub fn update_text(&mut self, text: &str, size: f32, position: &LogicalPosition<u32>) {
    let fonts = Assets::get_font_list();
    let mut layout_settings = *self.layout.settings();

    if layout_settings.x != position.x as f32 && layout_settings.y != position.y as f32 {
      layout_settings.x = position.x as f32;
      layout_settings.y = position.y as f32;
    }

    self.layout.reset(&layout_settings);

    let font_index = self.font_index().unwrap_or(0);
    let style = TextStyle::new(text, size, font_index);

    self.layout.append(fonts, &style);
    self.dimensions = Self::calculate_dimensions(self.layout.glyphs(), position);
  }

  pub fn update_font_index(&mut self, new_font_index: usize) -> anyhow::Result<()> {
    let fonts = Assets::get_font_list();

    if fonts.len() < new_font_index + 1 {
      return Err(anyhow!(
        "Attempted to update a text box with an invalid font index."
      ));
    }

    let text = self.text();
    let Some(size) = self.px() else {
      return Err(anyhow!("There is no text."));
    };
    let position = self.position();

    self.update_text(&text, size, &position);

    Ok(())
  }

  /// Returns the font index for the text within this textbox instance.
  ///
  /// None is returned if the text box is empty.
  pub fn font_index(&self) -> Option<usize> {
    Some(self.layout.glyphs().first()?.font_index)
  }

  /// Returns the stored top left of this text box instance.
  pub fn position(&self) -> LogicalPosition<u32> {
    let settings = self.layout.settings();

    (settings.x, settings.y).into()
  }

  /// Returns the text contained in this textbox instance.
  pub fn text(&self) -> String {
    self
      .layout
      .glyphs()
      .iter()
      .map(|glyph| glyph.parent)
      .collect()
  }

  /// Returns the size of the text in px for this textbox instance.
  ///
  /// None is returned if the textbox is empty.
  pub fn px(&self) -> Option<f32> {
    Some(self.layout.glyphs().first()?.key.px)
  }

  /// Returns the data about each character as [`GlyphPosition`](https://docs.rs/fontdue/0.8.0/fontdue/layout/struct.GlyphPosition.html)
  pub fn glyphs(&self) -> &Vec<GlyphPosition<()>> {
    self.layout.glyphs()
  }

  pub fn height(&self) -> u32 {
    self.layout.height().cast()
  }

  pub fn dimensions(&self) -> &LogicalSize<u32> {
    &self.dimensions
  }

  fn calculate_dimensions(
    glyphs: &[GlyphPosition<()>],
    position: &LogicalPosition<u32>,
  ) -> LogicalSize<u32> {
    let mut largest_height = 0;
    let mut farthest_right = 0;
    let glyph_count = glyphs.len();

    if glyph_count == 0 {
      return LogicalSize::default();
    }

    glyphs.iter().enumerate().for_each(|(iteration, glyph)| {
      let glyph_height = glyph.height as u32;

      if iteration + 1 == glyph_count {
        farthest_right = glyph.x as u32 + glyph.width as u32
      }

      if glyph_height > largest_height {
        largest_height = glyph_height;
      }
    });

    let width = farthest_right - position.x;
    let height = largest_height;

    LogicalSize { width, height }
  }
}

impl Renderable for TextBox {
  /// Renders the text for the given [`TextBox`](crate::renderer::fonts::TextBox).
  fn render(
    &self,
    renderer: &mut Renderer,
    _position: &LogicalPosition<u32>,
    color: &[u8; 4],
  ) -> anyhow::Result<()> {
    let Some(font_index) = self.font_index() else {
      log::warn!("Attempted to render an empty text box.");

      return Ok(());
    };

    let Some(font) = Assets::get_font_list().get(font_index) else {
      return Err(anyhow!(
        "Attempted to obtain a font index that's out of bounds. Index {} > {}",
        font_index,
        Assets::get_font_list().len()
      ));
    };

    let buffer = renderer.pixels.frame_mut();
    let text_box_y = self.position().y;
    let text_box_height = self.dimensions().height;

    let result: anyhow::Result<()> = self.glyphs().iter().try_for_each(|glyph| {
      if !glyph.parent.is_ascii() {
        return Err(anyhow!(
          "Attempted to render a non-ascii character: `{:?}`",
          glyph.parent
        ));
      }

      let (metrics, bitmap) = font.rasterize(glyph.parent, glyph.key.px);
      let (char_width, char_height) = (glyph.width as u32, glyph.height as u32);

      // char_x + (((text_box_y + text_box_height) - (char_y_min + char_height)).max(0) * buffer_width)
      let top_left_placement = glyph.x.cast::<u32>()
        + (((text_box_y + text_box_height) as i32 - (metrics.ymin + metrics.height as i32)).max(0)
          as u32
          * renderer.buffer_dimensions.width);

      for index in 0..(char_width * char_height) {
        let position = top_left_placement
          + (index % char_width)
          + ((index / char_width) * renderer.buffer_dimensions.width);

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

        Renderer::draw_at_pixel_with_rgba(buffer, position as usize, &color)?;
      }

      Ok(())
    });

    if let Err(error) = result {
      return Err(anyhow!("Failed to render the text. `{:?}`", error));
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dimensions_are_calculated_correctly() {
    let text = "Pause:";
    let size = 20.0;
    let position = LogicalPosition::default();
    let text_box = TextBox::new(0, text, &position, size).unwrap();

    let expected_dimensions: LogicalSize<u32> = LogicalSize::new(115, 30);

    let dimensions = TextBox::calculate_dimensions(text_box.layout.glyphs(), &position);

    assert_eq!(dimensions, expected_dimensions);
  }

  mod test_data {}
}
