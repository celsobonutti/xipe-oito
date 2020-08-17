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
  SkipIfRegisterEqual(u8, u8),
  SetRegister {
    target_register: u8,
    value: u8
  },
  IncreaseRegister {
    target_register: u8,
    value: u8
  },
  SetByRegister(u8, u8),
  SetXOrY(u8, u8),
  SetXAndY(u8, u8),
  SetXXORY(u8, u8),
  AddsYToX(u8, u8),
  SubsYFromX(u8, u8),
  ShiftRight(u8),
  SetXAsYMinusX(u8, u8),
  ShiftLeft(u8),
  SkipIfRegisterDifferent(u8, u8),
  SetsITo(u16),
  JumpsToNPlusV0(u16),
  Random(u8, u8),
  Display(u8, u8, u8),
  SkipIfKeyPressed(u8),
  SkipIfKeyNotPressed(u8),
  SetXAsDelay(u8),
  SetXAsKey(u8),
  SetDelayAsX(u8),
  SetSoundAsX(u8),
  AddsXToI(u8),
  SetsIAsSprite(u8),
  StoreBCD(u8),
  DumpRegisters(u8),
  LoadRegisters(u8)
}

pub fn decode(op_code: u16) {
  match (format!("{:x}", op_code)) {

  }
}