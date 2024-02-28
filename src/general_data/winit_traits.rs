use winit::event::Event;
use winit::window::WindowId;

pub trait EventWindowId {
  fn get_window_id(&self) -> Option<WindowId>;
}

impl<T> EventWindowId for Event<T> {
  fn get_window_id(&self) -> Option<WindowId> {
    if let Event::WindowEvent { window_id, .. } = self {
      return Some(*window_id);
    }

    None
  }
}

pub trait Add {
  fn add(&self, other: &Self) -> Self;
}

impl Add for winit::dpi::PhysicalPosition<u32> {
  fn add(&self, other: &Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

pub trait Mul {
  fn multiply(&self, other: u32) -> Self;
}

impl Mul for winit::dpi::LogicalSize<u32> {
  fn multiply(&self, other: u32) -> Self {
    Self {
      width: self.width * other,
      height: self.height * other,
    }
  }
}
