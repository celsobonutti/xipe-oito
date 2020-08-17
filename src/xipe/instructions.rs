enum Instruction {
  DisplayClear,
  Return,
  GoTo(u16),
  Call(u16),
  SkipIfEqual {
    register: u8,
    value: u8
  },
  SkipIfDifferent {
    register: u8,
    value: u8
  },
  SkipIfRegisterEqual {
    register_one: u8,
    register_two: u8
  },
  SetRegister {
    register: u8,
    value: u8
  },
  IncreaseRegister {
    register: u8,
    value: u8
  },
  
}

pub fn decode(op_code: u16) {
  match (format!("{:x}", op_code)) {

  }
}