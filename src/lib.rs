mod xipe;

use crate::xipe::processor;

pub fn start() -> processor::Chip8 {
  processor::Chip8::new()
}