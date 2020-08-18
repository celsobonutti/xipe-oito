use byteorder::{BigEndian, ReadBytesExt};
use super::instructions::{self, Instruction};

pub const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

pub struct Chip8 {
  pub operation_code: u16,
  pub memory: [u8; 4096],
  pub registers: [u8; 16],
  pub index: u16,
  pub program_counter: u16,
  pub gfx: [bool; 64 * 32],
  delay_timer: u8,
  sound_timer: u8,
  stack: [u16; 16],
  stack_pointer: u16,
  key: [bool; 16]
}

impl Chip8 {
  pub fn new() -> Chip8 {
    let mut memory = [0; 4096];
    
    for (index, character) in FONTSET.iter().enumerate() {
      memory[index] = *character;
    };

    Chip8 {
      operation_code: 0,
      memory,
      registers: [0; 16],
      index: 0,
      program_counter: 0x200,
      gfx: [false; 64 * 32],
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      stack_pointer: 0,
      key: [false; 16]
    }
  }

  pub fn load(&mut self, buffer: Vec<u8>) {
    for (index, value) in buffer.iter().enumerate() {
      self.memory[index + 512] = *value;
    }
  }

  fn emulate_cycle(&mut self) {
    let position = self.program_counter as usize;
    let mut opcode = &self.memory[position..position+2];
    let op_code = opcode.read_u16::<BigEndian>().unwrap();
    match instructions::decode(op_code) {
      Instruction::ClearDisplay => {
        self.gfx = [false; 64 * 32]
      }
      
    }
  }
}