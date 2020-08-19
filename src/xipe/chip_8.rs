use super::instructions::{self, Instruction, RegisterValuePair, TargetSourcePair};
use byteorder::{BigEndian, ReadBytesExt};

pub const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
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
    stack_pointer: usize,
    key: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut memory = [0; 4096];

        for (index, character) in FONTSET.iter().enumerate() {
            memory[index] = *character;
        }

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
            key: [false; 16],
        }
    }

    pub fn load(&mut self, buffer: Vec<u8>) {
        for (index, value) in buffer.iter().enumerate() {
            self.memory[index + 512] = *value;
        }
    }

    fn increment_pc(&mut self) {
        self.program_counter += 2;
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

    fn pop_stack(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    fn skip_if(&mut self, condition: bool) {
        if condition {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn execute_op(&mut self, op_code: u16) {
        match instructions::decode(op_code) {
            Instruction::CallMachineCode(_) => {
                println!("Do you have me or something?");
            }
            Instruction::ClearDisplay => {
                self.gfx = [false; 64 * 32];
                self.increment_pc();
            }
            Instruction::Return => {
                self.pop_stack();
            }
            Instruction::GoTo(addr) => {
                self.program_counter = addr;
            }
            Instruction::GoToNPlusV0(addr) => {
                self.program_counter = addr + self.get_register(0x0) as u16;
            }
            Instruction::Call(addr) => {
                self.push_stack();
                self.program_counter = addr;
            }
            Instruction::AssignValueToRegister(RegisterValuePair { register, value }) => {
                self.set_register(register, value);
                self.increment_pc();
            }
            Instruction::AssignVYToVX(TargetSourcePair { target, source }) => {
                self.set_register(target, self.get_register(source));
                self.increment_pc();
            }
            Instruction::AddValueToRegister(RegisterValuePair { register, value }) => {
                let (sum, _) = self.get_register(register).overflowing_add(value);
                self.set_register(register, sum);
                self.increment_pc();
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
                self.increment_pc();
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
                self.increment_pc();
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
                self.increment_pc();
            }
            Instruction::SetXOrY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) | self.get_register(source);
                self.set_register(target, result);
                self.increment_pc();
            }
            Instruction::SetXAndY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) & self.get_register(source);
                self.set_register(target, result);
                self.increment_pc();
            }
            Instruction::SetXXorY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) ^ self.get_register(source);
                self.set_register(target, result);
                self.increment_pc();
            }
            Instruction::DumpRegisters(limit) => {
                for i in 0..limit + 1 {
                    self.set_memory(self.index, self.get_register(i));
                    self.index += 1;
                }
                self.increment_pc();
            }
            Instruction::LoadRegisters(limit) => {
                for i in 0..limit + 1 {
                    self.set_register(i, self.get_memory(self.index));
                    self.index += 1;
                }
                self.increment_pc();
            }
            Instruction::SetDelayAsX(register) => {
                self.delay_timer = self.get_register(register);
                self.increment_pc();
            }
            Instruction::SetSoundAsX(register) => {
                self.sound_timer = self.get_register(register);
                self.increment_pc();
            }
            Instruction::SetIAs(value) => {
                self.index = value;
                self.increment_pc();
            }
            Instruction::AddXToI(register) => {
                let (result, _) = self
                    .index
                    .overflowing_add(self.get_register(register) as u16);
                self.index = result;
                self.increment_pc();
            }
            Instruction::SkipIfEqual(RegisterValuePair { register, value }) => {
                self.skip_if(self.get_register(register) == value);
            }
            Instruction::SkipIfDifferent(RegisterValuePair { register, value }) => {
                self.skip_if(self.get_register(register) != value);
            }
            Instruction::SkipIfRegisterEqual(TargetSourcePair { target, source }) => {
                self.skip_if(self.get_register(target) == self.get_register(source));
            }
            Instruction::SkipIfRegisterDifferent(TargetSourcePair { target, source }) => {
                self.skip_if(self.get_register(target) != self.get_register(source));
            }
        }
    }

    fn emulate_cycle(&mut self) {
        let position = self.program_counter as usize;
        let mut op_code = &self.memory[position..position + 2];
        let op_code = op_code.read_u16::<BigEndian>().unwrap();

        self.execute_op(op_code);

        if self.delay_timer > 0 {
            self.delay_timer -= 1
        };

        match self.sound_timer {
            0 => {}
            1 => println!("Bip bop boop"),
            _ => self.sound_timer -= 1,
        }
    }
}
