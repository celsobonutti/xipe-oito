mod display;
mod fontset;
mod input;
mod instructions;
mod processor;

pub fn new(emit_sound: Box<dyn Fn()>, ) -> processor::Chip8 {
  processor::Chip8::new(emit_sound)
} 
