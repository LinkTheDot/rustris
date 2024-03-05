use crate::renderer::Renderer;
use anyhow::anyhow;
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle};
use winit::dpi::*;

pub struct TextBox {
  layout: Layout,
}

impl std::fmt::Debug for TextBox {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "TextBox {{ .. }}")
  }
}

impl TextBox {
  pub fn new(
    renderer: &Renderer,
    font_index: usize,
    text: &str,
    position: &LogicalPosition<u32>,
    size: f32,
  ) -> Self {
    let style = TextStyle::new(text, size, font_index);

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    let layout_settings = LayoutSettings {
      x: position.x as f32,
      y: position.y as f32,
      ..Default::default()
    };
    layout.reset(&layout_settings);

    layout.append(renderer.fonts(), &style);

    Self { layout }
  }

  /// Updates the text contained in this textbox.
  ///
  /// If the textbox was originally an empty string, the font_index is set to 0.
  pub fn update_text(
    &mut self,
    renderer: &Renderer,
    text: &str,
    size: f32,
    position: &LogicalPosition<u32>,
  ) {
    let mut layout_settings = *self.layout.settings();

    if layout_settings.x != position.x as f32 && layout_settings.y != position.y as f32 {
      layout_settings.x = position.x as f32;
      layout_settings.y = position.y as f32;
    }

    self.layout.reset(&layout_settings);

    let font_index = self.font_index().unwrap_or(0);
    let style = TextStyle::new(text, size, font_index);

    self.layout.append(renderer.fonts(), &style);
  }

  pub fn update_font_index(
    &mut self,
    renderer: &Renderer,
    new_font_index: usize,
  ) -> anyhow::Result<()> {
    let fonts = renderer.fonts();

    if fonts.len() < new_font_index + 1 {
      return Err(anyhow!(
        "Attempted to update a text box with an invalid font index."
      ));
    }

    let text = self.text();
    let Some(size) = self.text_size() else {
      return Err(anyhow!("There is no text."));
    };
    let Some(position) = self.position() else {
      return Err(anyhow!("There is no text."));
    };

    self.update_text(renderer, &text, size, &position);

    Ok(())
  }

  /// Returns the font index for the text within this textbox instance.
  ///
  /// None is returned if the text box is empty.
  pub fn font_index(&self) -> Option<usize> {
    Some(self.layout.glyphs().first()?.font_index)
  }

  /// Retrns the top left most position of this textbox instance.
  ///
  /// None is returned if the text box is empty.
  pub fn position(&self) -> Option<LogicalPosition<u32>> {
    let glyph = self.layout.glyphs().first()?;

    Some(LogicalPosition {
      x: glyph.x.cast::<u32>(),
      y: glyph.y.cast::<u32>(),
    })
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
  pub fn text_size(&self) -> Option<f32> {
    Some(self.layout.glyphs().first()?.key.px)
  }

  /// Returns the data about each character as [`GlyphPosition`](https://docs.rs/fontdue/0.8.0/fontdue/layout/struct.GlyphPosition.html)
  pub fn character_data(&self) -> &Vec<GlyphPosition<()>> {
    self.layout.glyphs()
  }
}
