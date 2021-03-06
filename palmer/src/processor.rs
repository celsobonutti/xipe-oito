use byteorder::{BigEndian, ReadBytesExt};

use super::audio::AudioDriver;
use super::display::Display;
use super::fontset::FONTSET;
use super::input::Input;
use super::instructions::{self, Instruction, RegisterValuePair, TargetSourcePair};

const MEMORY_SIZE: usize = 4096;
const OP_SIZE: u16 = 2;

#[cfg(target_arch = "wasm32")]
fn get_random() -> u8 {
  unsafe { js_sys::Math::floor(js_sys::Math::random() * 255.) as u8 }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_random() -> u8 {
  rand::random()
}

enum ProgramCounter {
  Next,
  Skip,
  Jump(u16),
}

fn skip_if(condition: bool) -> ProgramCounter {
  if condition {
    ProgramCounter::Skip
  } else {
    ProgramCounter::Next
  }
}

pub struct Chip8<T: AudioDriver> {
  pub display: Display,
  pub input: Input,
  memory: [u8; MEMORY_SIZE],
  registers: [u8; 16],
  index: u16,
  program_counter: u16,
  delay_timer: u8,
  sound_timer: u8,
  stack: [u16; 16],
  stack_pointer: usize,
  waiting_for_key: Option<u8>,
  audio_driver: T,
  should_draw: bool,
}

impl<T: AudioDriver> Chip8<T> {
  pub fn new(audio_driver: T) -> Chip8<T> {
    let mut memory = [0; MEMORY_SIZE];

    for (index, character) in FONTSET.iter().enumerate() {
      memory[index] = *character;
    }

    Chip8 {
      memory,
      registers: [0; 16],
      index: 0,
      program_counter: 0x200,
      display: Display::new(),
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      stack_pointer: 0,
      input: Input::new(),
      waiting_for_key: None,
      should_draw: false,
      audio_driver,
    }
  }

  pub fn load(&mut self, buffer: Vec<u8>) {
    for (index, value) in buffer.iter().enumerate() {
      self.memory[index + 512] = *value;
    }
  }

  pub fn reset(&mut self) {
    for index in 512..MEMORY_SIZE {
      self.memory[index] = 0;
    }

    self.registers = [0; 16];
    self.index = 0;
    self.program_counter = 0x200;
    self.display = Display::new();
    self.delay_timer = 0;
    self.sound_timer = 0;
    self.stack = [0; 16];
    self.stack_pointer = 0;
    self.waiting_for_key = None;
  }

  pub fn should_draw(&self) -> bool {
    self.should_draw
  }

  fn set_register(&mut self, register: u8, value: u8) {
    self.registers[register as usize] = value;
  }

  fn set_memory(&mut self, index: u16, value: u8) {
    self.memory[index as usize] = value;
  }

  fn set_vf(&mut self, value: u8) {
    self.set_register(0xF, value);
  }

  fn get_register(&self, register: u8) -> u8 {
    self.registers[register as usize]
  }

  fn get_memory(&self, index: u16) -> u8 {
    self.memory[index as usize]
  }

  fn push_stack(&mut self) {
    self.stack[self.stack_pointer] = self.program_counter + 2;
    self.stack_pointer += 1;
  }

  fn pop_stack(&mut self) -> u16 {
    self.stack_pointer -= 1;
    self.stack[self.stack_pointer]
  }

  fn execute_op(&mut self, op_code: u16) -> ProgramCounter {
    match instructions::decode(op_code) {
      Instruction::CallMachineCode(_) => {
        println!("Do you hate me or something?");
        ProgramCounter::Next
      }
      Instruction::ClearDisplay => {
        self.display.clear();
        ProgramCounter::Next
      }
      Instruction::Return => ProgramCounter::Jump(self.pop_stack()),
      Instruction::GoTo(addr) => ProgramCounter::Jump(addr),
      Instruction::Call(addr) => {
        self.push_stack();
        ProgramCounter::Jump(addr)
      }
      Instruction::SkipIfEqual(RegisterValuePair { register, value }) => {
        skip_if(self.get_register(register) == value)
      }
      Instruction::SkipIfDifferent(RegisterValuePair { register, value }) => {
        skip_if(self.get_register(register) != value)
      }
      Instruction::SkipIfRegisterEqual(TargetSourcePair { target, source }) => {
        skip_if(self.get_register(target) == self.get_register(source))
      }
      Instruction::AssignValueToRegister(RegisterValuePair { register, value }) => {
        self.set_register(register, value);
        ProgramCounter::Next
      }
      Instruction::AddValueToRegister(RegisterValuePair { register, value }) => {
        let (sum, _) = self.get_register(register).overflowing_add(value);
        self.set_register(register, sum);
        ProgramCounter::Next
      }
      Instruction::AssignVYToVX(TargetSourcePair { target, source }) => {
        self.set_register(target, self.get_register(source));
        ProgramCounter::Next
      }
      Instruction::SetXOrY(TargetSourcePair { target, source }) => {
        let result = self.get_register(target) | self.get_register(source);
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::SetXAndY(TargetSourcePair { target, source }) => {
        let result = self.get_register(target) & self.get_register(source);
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::SetXXorY(TargetSourcePair { target, source }) => {
        let result = self.get_register(target) ^ self.get_register(source);
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::AddYToX(TargetSourcePair { target, source }) => {
        let (result, did_overflow) = self
          .get_register(target)
          .overflowing_add(self.get_register(source));
        if did_overflow {
          self.set_vf(1);
        } else {
          self.set_vf(0);
        }
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::SubYFromX(TargetSourcePair { target, source }) => {
        let (result, did_overflow) = self
          .get_register(target)
          .overflowing_sub(self.get_register(source));
        if did_overflow {
          self.set_vf(0)
        } else {
          self.set_vf(1)
        }
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::ShiftRight(register) => {
        let reg_value = self.get_register(register);
        self.set_vf(reg_value & 0b1);
        self.set_register(register, reg_value >> 1);
        ProgramCounter::Next
      }
      Instruction::SetXAsYMinusX(TargetSourcePair { target, source }) => {
        let (result, did_overflow) = self
          .get_register(source)
          .overflowing_sub(self.get_register(target));
        if did_overflow {
          self.set_vf(0)
        } else {
          self.set_vf(1)
        }
        self.set_register(target, result);
        ProgramCounter::Next
      }
      Instruction::ShiftLeft(register) => {
        let reg_value = self.get_register(register);
        self.set_vf((reg_value & 0b10000000) / 128);
        self.set_register(register, reg_value << 1);
        ProgramCounter::Next
      }
      Instruction::SkipIfRegisterDifferent(TargetSourcePair { target, source }) => {
        skip_if(self.get_register(target) != self.get_register(source))
      }
      Instruction::SetIAs(value) => {
        self.index = value;
        ProgramCounter::Next
      }
      Instruction::GoToNPlusV0(addr) => ProgramCounter::Jump(addr + self.get_register(0x0) as u16),
      Instruction::Random(RegisterValuePair { register, value }) => {
        let rnd: u8 = get_random();
        self.set_register(register, rnd & value);
        ProgramCounter::Next
      }
      Instruction::Draw { x, y, height } => {
        let new_vf = self.display.draw(
          self.get_register(x) as usize,
          self.get_register(y) as usize,
          &self.memory[self.index as usize..(self.index + height as u16) as usize],
        );
        self.set_vf(new_vf);
        self.should_draw = true;
        ProgramCounter::Next
      }
      Instruction::SkipIfKeyPressed(register) => {
        let reg_value = self.get_register(register);
        skip_if(self.input.is_pressed(reg_value))
      }
      Instruction::SkipIfKeyNotPressed(register) => {
        let reg_value = self.get_register(register);
        skip_if(!self.input.is_pressed(reg_value))
      }
      Instruction::SetXAsDelay(register) => {
        self.set_register(register, self.delay_timer);
        ProgramCounter::Next
      }
      Instruction::WaitForInputAndStoreIn(register) => {
        self.waiting_for_key = Some(register);
        ProgramCounter::Next
      }
      Instruction::SetDelayAsX(register) => {
        self.delay_timer = self.get_register(register);
        ProgramCounter::Next
      }
      Instruction::SetSoundAsX(register) => {
        self.sound_timer = self.get_register(register);
        ProgramCounter::Next
      }
      Instruction::AddXToI(register) => {
        let (result, _) = self
          .index
          .overflowing_add(self.get_register(register) as u16);
        self.index = result;
        ProgramCounter::Next
      }
      Instruction::StoreBCD(register) => {
        let value = self.get_register(register);
        self.set_memory(self.index, value / 100);
        self.set_memory(self.index + 1, (value % 100) / 10);
        self.set_memory(self.index + 2, value % 10);
        ProgramCounter::Next
      }
      Instruction::DumpRegisters(limit) => {
        for i in 0..=limit {
          self.set_memory(self.index, self.get_register(i));
          self.index += 1;
        }
        ProgramCounter::Next
      }
      Instruction::LoadRegisters(limit) => {
        for i in 0..=limit {
          self.set_register(i, self.get_memory(self.index));
          self.index += 1;
        }
        ProgramCounter::Next
      }
      Instruction::SetIAsFontSprite(register) => {
        self.index = self.get_register(register) as u16 * 5;
        ProgramCounter::Next
      }
      Instruction::InvalidInstruction => ProgramCounter::Next,
    }
  }

  pub fn emulate_cycle(&mut self) {
    if self.should_draw {
      self.should_draw = false;
    }
    if let Some(register) = self.waiting_for_key {
      if let Some(index) = self.input.keypad.iter().position(|val| *val) {
        self.waiting_for_key = None;
        self.set_register(register, index as u8);
      }
    } else {
      let position = self.program_counter as usize;
      let mut op_code = &self.memory[position..position + 2];
      let op_code = op_code.read_u16::<BigEndian>().unwrap();

      let pg_op = self.execute_op(op_code);

      self.program_counter = match pg_op {
        ProgramCounter::Next => self.program_counter + OP_SIZE,
        ProgramCounter::Skip => self.program_counter + 2 * OP_SIZE,
        ProgramCounter::Jump(addr) => addr,
      };

      if self.delay_timer > 0 {
        self.delay_timer -= 1
      };

      match self.sound_timer {
        0 => {}
        1 => {
          self.audio_driver.play_sound();
          self.sound_timer -= 1
        }
        _ => self.sound_timer -= 1,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  struct TAD {
    pub is_playing: bool,
  }

  impl AudioDriver for TAD {
    fn new() -> Self {
      Self { is_playing: false }
    }

    fn play_sound(&mut self) {
      self.is_playing = true;
    }
  }

  use super::*;

  fn emulate_cycles(chip: &mut Chip8<TAD>, number_of_cycles: usize) {
    for _ in 0..number_of_cycles {
      chip.emulate_cycle();
    }
  }

  #[test]
  fn load_cartridge_and_reset() {
    let mut chip8 = Chip8::new(TAD::new());
    chip8.load(vec![0xFF, 0xF1, 0x01, 0x22]);
    assert_eq!(chip8.memory[512..=515], [0xFF, 0xF1, 0x01, 0x22]);
    chip8.reset();
    for index in 512..MEMORY_SIZE {
      assert_eq!(chip8.get_memory(index as u16), 0);
    }
  }

  #[test]
  fn call_subroutine_return_and_jump() {
    let mut chip8 = Chip8::new(TAD::new());
    chip8.load(vec![0x22, 0x04, 0x12, 0x00, 0x00, 0xEE]);
    chip8.emulate_cycle();
    assert_eq!(chip8.stack[0], 0x202);
    assert_eq!(chip8.stack_pointer, 1);
    assert_eq!(chip8.program_counter, 0x204);
    chip8.emulate_cycle();
    assert_eq!(chip8.stack_pointer, 0);
    assert_eq!(chip8.program_counter, 0x202);
    chip8.emulate_cycle();
    assert_eq!(chip8.program_counter, 0x200);
  }

  #[test]
  fn vx_operations() {
    let mut chip8 = Chip8::new(TAD::new());

    let instructions = vec![
      0x61, 0xF0, // v1 = 0xf0
      0x71, 0x11, // v1 = 0xf0 + 0x11
      0x82, 0x10, // v2 = v1
      0x61, 0xF0, // v1 = 0xf0
      0x62, 0x11, // v2 = 0x11
      0x81, 0x21, // v1 = v1 | v2 => 0xf1
      0x81, 0x22, // v1 = v1 & v2 => 0x11
      0x61, 0x21, // v1 = 0x21
      0x81, 0x23, // v1 = v1 ^ v2 => 0x30
      0x61, 0xF0, // v1 = 0xf0
      0x81, 0x24, // v1 = v1 + v2 => 0x01; vf = 0x01
      0x81, 0x25, // v1 = v1 - v2 => 0xf0; vf = 0x00
    ];

    chip8.load(instructions);

    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0xF0);
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0x01);
    assert_eq!(chip8.get_register(0xf), 0x00);
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), chip8.get_register(2));
    chip8.emulate_cycle();
    chip8.emulate_cycle();
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0xf1);
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0x11);
    chip8.emulate_cycle();
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0x30);
    chip8.emulate_cycle();
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0x01);
    assert_eq!(chip8.get_register(0xf), 0x01);
    chip8.emulate_cycle();
    assert_eq!(chip8.get_register(1), 0xf0);
    assert_eq!(chip8.get_register(0xf), 0x00);
  }

  #[test]
  fn set_i_register() {
    let mut chip8 = Chip8::new(TAD::new());

    let instructions = vec![
      0xA5, 0x00, 0x60, 0x05, 0xF0, 0x1E, 0x60, 0x03, 0xF0, 0x29, 0xA5, 0x00, 0x60, 218, 0xF0, 0x33,
    ];

    chip8.load(instructions);

    assert_eq!(chip8.index, 0x0);

    chip8.emulate_cycle();

    assert_eq!(chip8.index, 0x500);

    emulate_cycles(&mut chip8, 2);

    assert_eq!(chip8.index, 0x505);

    emulate_cycles(&mut chip8, 2);

    assert_eq!(chip8.index, 15);

    emulate_cycles(&mut chip8, 3);

    assert_eq!(chip8.get_memory(chip8.index), 2);
    assert_eq!(chip8.get_memory(chip8.index + 1), 1);
    assert_eq!(chip8.get_memory(chip8.index + 2), 8);
  }

  #[test]
  fn dump_and_load_registers() {
    let mut chip8 = Chip8::new(TAD::new());

    let instructions = vec![
      0xA4, 0x00, 0x60, 0xF0, 0x61, 0xDD, 0x62, 0x1E, 0x63, 0x17, 0x64, 0x4D, 0x65, 0x29, 0xF5,
      0x55, 0x60, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00, 0x64, 0x00, 0x65, 0x00, 0xA4, 0x00,
      0xF5, 0x65,
    ];

    chip8.load(instructions);

    chip8.emulate_cycle();

    assert_eq!(chip8.index, 0x400);

    emulate_cycles(&mut chip8, 6);

    chip8.emulate_cycle();

    assert_eq!(chip8.index, 0x406);

    for index in 0..=5 {
      assert_eq!(
        chip8.get_register(index),
        chip8.get_memory(0x400 + index as u16)
      );
    }

    emulate_cycles(&mut chip8, 6);

    for index in 0..=5 {
      assert_eq!(chip8.get_register(index), 0x0);
    }

    emulate_cycles(&mut chip8, 2);

    for index in 0..=5 {
      assert_eq!(
        chip8.get_register(index),
        chip8.get_memory(0x400 + index as u16)
      );
    }
  }

  #[test]
  fn timers() {
    let mut chip8 = Chip8::new(TAD::new());

    let instructions = vec![0x60, 0x02, 0xF0, 0x15, 0xF0, 0x18];

    chip8.load(instructions);

    chip8.emulate_cycle();

    chip8.emulate_cycle();

    assert_eq!(chip8.delay_timer, 1);

    chip8.emulate_cycle();

    assert_eq!(chip8.delay_timer, 0);
    assert_eq!(chip8.sound_timer, 1);

    chip8.emulate_cycle();

    assert!(chip8.audio_driver.is_playing);
    assert_eq!(chip8.sound_timer, 0);
  }
}
