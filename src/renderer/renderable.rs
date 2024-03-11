use crate::renderer::Renderer;
use winit::dpi::LogicalPosition;

pub trait Renderable {
  fn render(
    &self,
    renderer: &mut Renderer,
    position: &LogicalPosition<u32>,
    color: &[u8; 4],
  ) -> anyhow::Result<()>;
}
