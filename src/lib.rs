mod xipe;

use crate::xipe::processor;

fn emit_sound() {
  println!("Beep bop boop");
}

pub fn start() -> processor::Chip8 {
  processor::Chip8::new(
    emit_sound
  )
}