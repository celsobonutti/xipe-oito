mod xipe;

use crate::xipe::chip_8;

pub fn start() -> chip_8::Chip8 {
  chip_8::Chip8::new()
}