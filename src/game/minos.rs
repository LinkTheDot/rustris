#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MinoType {
  I,
  L,
  J,
  O,
  T,
  S,
  Z,
}

impl MinoType {
  #[inline]
  pub fn color(&self) -> [u8; 3] {
    self.into()
  }
}

impl From<&MinoType> for [u8; 3] {
  fn from(mino: &MinoType) -> [u8; 3] {
    match mino {
      MinoType::I => [0x32, 0xC5, 0xF5],
      MinoType::L => [0xFA, 0xA2, 0x47],
      MinoType::J => [0x00, 0x7A, 0xBF],
      MinoType::O => [0xFE, 0xD7, 0x1E],
      MinoType::T => [0xA0, 0x51, 0x9F],
      MinoType::S => [0x7B, 0xBE, 0x44],
      MinoType::Z => [0xEF, 0x4B, 0x39],
    }
  }
}
