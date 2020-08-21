use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;

use super::display::Display;
use super::fontset::FONTSET;
use super::input::Input;
use super::instructions::{self, Instruction, RegisterValuePair, TargetSourcePair};

const MEMORY_SIZE: usize = 4096;
const OP_SIZE: u16 = 2;

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

pub struct Chip8 {
    pub operation_code: u16,
    pub memory: [u8; MEMORY_SIZE],
    pub registers: [u8; 16],
    pub index: u16,
    pub program_counter: u16,
    pub display: Display,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: usize,
    input: Input,
    waiting_for_key: Option<u8>,
    on_buzz: fn(),
}

impl Chip8 {
    pub fn new(on_buzz: fn()) -> Chip8 {
        let mut memory = [0; MEMORY_SIZE];

        for (index, character) in FONTSET.iter().enumerate() {
            memory[index] = *character;
        }

        Chip8 {
            operation_code: 0,
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
            on_buzz,
        }
    }

    pub fn load(&mut self, buffer: Vec<u8>) {
        for (index, value) in buffer.iter().enumerate() {
            self.memory[index + 512] = *value;
        }
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
            Instruction::GoToNPlusV0(addr) => {
                ProgramCounter::Jump(addr + self.get_register(0x0) as u16)
            }
            Instruction::Random(RegisterValuePair { register, value }) => {
                let mut rng = rand::thread_rng();
                let rnd: u8 = rng.gen();
                self.set_register(register, rnd & value);
                ProgramCounter::Next
            }
            Instruction::Draw { x, y, height } => {
                self.set_vf(0x0);
                self.display.draw(
                    x as usize,
                    y as usize,
                    &self.memory[self.index as usize..(self.index + height as u16) as usize],
                );
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
                for i in 0..limit + 1 {
                    self.set_memory(self.index, self.get_register(i));
                    self.index += 1;
                }
                ProgramCounter::Next
            }
            Instruction::LoadRegisters(limit) => {
                for i in 0..limit + 1 {
                    self.set_register(i, self.get_memory(self.index));
                    self.index += 1;
                }
                ProgramCounter::Next
            }
            Instruction::InvalidInstruction => ProgramCounter::Next,
            _ => ProgramCounter::Next
        }
    }

    fn emulate_cycle(&mut self) {
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
                1 => (self.on_buzz)(),
                _ => self.sound_timer -= 1,
            }
        }
    }
}
