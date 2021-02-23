macro_rules! hex_group_to_integer {
  ( $nibble1:expr ) => {
    $nibble1 as u8
  };
  ( $nibble1:expr, $nibble2:expr ) => {{
    (($nibble1 << 4) as u8) | $nibble2
  }};
  ( $nibble1:expr, $nibble2:expr, $nibble3:expr, $nibble4:expr ) => {{
    (($nibble1 as u16) << 12)
      | (($nibble2 as u16) << 8)
      | (($nibble3 as u16) << 4)
      | ($nibble4 as u16)
  }};
  ( $nibble1:expr, $nibble2:expr, $nibble3:expr ) => {{
    hex_group_to_integer!(0x0, $nibble1, $nibble2, $nibble3)
  }};
}

#[derive(Debug, PartialEq)]
pub struct TargetSourcePair {
  pub target: u8,
  pub source: u8,
}

#[derive(Debug, PartialEq)]
pub struct RegisterValuePair {
  pub register: u8,
  pub value: u8,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
  CallMachineCode(u16),
  ClearDisplay,
  Return,
  GoTo(u16),
  Call(u16),
  SkipIfEqual(RegisterValuePair),
  SkipIfDifferent(RegisterValuePair),
  SkipIfRegisterEqual(TargetSourcePair),
  AssignValueToRegister(RegisterValuePair),
  AddValueToRegister(RegisterValuePair),
  AssignVYToVX(TargetSourcePair),
  SetXOrY(TargetSourcePair),
  SetXAndY(TargetSourcePair),
  SetXXorY(TargetSourcePair),
  AddYToX(TargetSourcePair),
  SubYFromX(TargetSourcePair),
  ShiftRight(u8),
  SetXAsYMinusX(TargetSourcePair),
  ShiftLeft(u8),
  SkipIfRegisterDifferent(TargetSourcePair),
  SetIAs(u16),
  GoToNPlusV0(u16),
  Random(RegisterValuePair),
  Draw { x: u8, y: u8, height: u8 },
  SkipIfKeyPressed(u8),
  SkipIfKeyNotPressed(u8),
  SetXAsDelay(u8),
  WaitForInputAndStoreIn(u8),
  SetDelayAsX(u8),
  SetSoundAsX(u8),
  AddXToI(u8),
  SetIAsFontSprite(u8),
  StoreBCD(u8),
  DumpRegisters(u8),
  LoadRegisters(u8),
  InvalidInstruction,
}

fn as_ts_pair(x: u8, y: u8) -> TargetSourcePair {
  TargetSourcePair {
    target: hex_group_to_integer!(x),
    source: hex_group_to_integer!(y),
  }
}

fn as_rv_pair(register: u8, c1: u8, c2: u8) -> RegisterValuePair {
  RegisterValuePair {
    register: hex_group_to_integer!(register),
    value: hex_group_to_integer!(c1, c2),
  }
}

fn as_nibble_array(op_code: u16) -> [u8; 4] {
  let first_nibble = ((op_code >> 12) & 0xF) as u8;
  let second_nibble = ((op_code >> 8) & 0xF) as u8;
  let third_nibble = ((op_code >> 4) & 0xF) as u8;
  let fourth_nibble = (op_code & 0xF) as u8;
  [first_nibble, second_nibble, third_nibble, fourth_nibble]
}

pub fn decode(op_code: u16) -> Instruction {
  let nibble_array = as_nibble_array(op_code);

  match nibble_array {
    [0x0, 0x0, 0xE, 0x0] => Instruction::ClearDisplay,
    [0x0, 0x0, 0xE, 0xE] => Instruction::Return,
    [0x0, c1, c2, c3] => Instruction::CallMachineCode(hex_group_to_integer!(c1, c2, c3)),
    [0x1, c1, c2, c3] => Instruction::GoTo(hex_group_to_integer!(c1, c2, c3)),
    [0x2, c1, c2, c3] => Instruction::Call(hex_group_to_integer!(c1, c2, c3)),
    [0x3, register, c1, c2] => Instruction::SkipIfEqual(as_rv_pair(register, c1, c2)),
    [0x4, register, c1, c2] => Instruction::SkipIfDifferent(as_rv_pair(register, c1, c2)),
    [0x5, x, y, 0x0] => Instruction::SkipIfRegisterEqual(as_ts_pair(x, y)),
    [0x6, register, c1, c2] => Instruction::AssignValueToRegister(as_rv_pair(register, c1, c2)),
    [0x7, register, c1, c2] => Instruction::AddValueToRegister(as_rv_pair(register, c1, c2)),
    [0x8, x, y, 0x0] => Instruction::AssignVYToVX(as_ts_pair(x, y)),
    [0x8, x, y, 0x1] => Instruction::SetXOrY(as_ts_pair(x, y)),
    [0x8, x, y, 0x2] => Instruction::SetXAndY(as_ts_pair(x, y)),
    [0x8, x, y, 0x3] => Instruction::SetXXorY(as_ts_pair(x, y)),
    [0x8, x, y, 0x4] => Instruction::AddYToX(as_ts_pair(x, y)),
    [0x8, x, y, 0x5] => Instruction::SubYFromX(as_ts_pair(x, y)),
    [0x8, x, _, 0x6] => Instruction::ShiftRight(hex_group_to_integer!(x)),
    [0x8, x, y, 0x7] => Instruction::SetXAsYMinusX(as_ts_pair(x, y)),
    [0x8, x, _, 0xE] => Instruction::ShiftLeft(hex_group_to_integer!(x)),
    [0x9, x, y, 0x0] => Instruction::SkipIfRegisterDifferent(as_ts_pair(x, y)),
    [0xA, c1, c2, c3] => Instruction::SetIAs(hex_group_to_integer!(c1, c2, c3)),
    [0xB, c1, c2, c3] => Instruction::GoToNPlusV0(hex_group_to_integer!(c1, c2, c3)),
    [0xC, register, c1, c2] => Instruction::Random(as_rv_pair(register, c1, c2)),
    [0xD, x, y, height] => Instruction::Draw {
      x: hex_group_to_integer!(x),
      y: hex_group_to_integer!(y),
      height: hex_group_to_integer!(height),
    },
    [0xE, x, 0x9, 0xE] => Instruction::SkipIfKeyPressed(hex_group_to_integer!(x)),
    [0xE, x, 0xA, 0x1] => Instruction::SkipIfKeyNotPressed(hex_group_to_integer!(x)),
    [0xF, x, 0x0, 0x7] => Instruction::SetXAsDelay(hex_group_to_integer!(x)),
    [0xF, x, 0x0, 0xA] => Instruction::WaitForInputAndStoreIn(hex_group_to_integer!(x)),
    [0xF, x, 0x1, 0x5] => Instruction::SetDelayAsX(hex_group_to_integer!(x)),
    [0xF, x, 0x1, 0x8] => Instruction::SetSoundAsX(hex_group_to_integer!(x)),
    [0xF, x, 0x1, 0xE] => Instruction::AddXToI(hex_group_to_integer!(x)),
    [0xF, x, 0x2, 0x9] => Instruction::SetIAsFontSprite(hex_group_to_integer!(x)),
    [0xF, x, 0x3, 0x3] => Instruction::StoreBCD(hex_group_to_integer!(x)),
    [0xF, x, 0x5, 0x5] => Instruction::DumpRegisters(hex_group_to_integer!(x)),
    [0xF, x, 0x6, 0x5] => Instruction::LoadRegisters(hex_group_to_integer!(x)),
    _ => Instruction::InvalidInstruction,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn convert_to_nibble_array() {
    let result = as_nibble_array(0xFA07);
    assert_eq!(result, [0xF, 0xA, 0x0, 0x7]);
  }

  #[test]
  fn group_macro_test() {
    assert_eq!(0xA, hex_group_to_integer!(0xA));
    assert_eq!(0xAB, hex_group_to_integer!(0xA, 0xB));
    assert_eq!(0x17B, hex_group_to_integer!(0x1, 0x7, 0xB));
    assert_eq!(0xF0DA, hex_group_to_integer!(0xF, 0x0, 0xD, 0xA))
  }

  #[test]
  fn call_machine_code() {
    assert_eq!(Instruction::CallMachineCode(0xABC), decode(0x0ABC))
  }

  #[test]
  fn clear_display() {
    assert_eq!(Instruction::ClearDisplay, decode(0x00E0))
  }

  #[test]
  fn return_from() {
    assert_eq!(Instruction::Return, decode(0x00EE))
  }

  #[test]
  fn go_to() {
    assert_eq!(Instruction::GoTo(0x0ABA), decode(0x1ABA))
  }

  #[test]
  fn call_at() {
    assert_eq!(Instruction::Call(0x0FBF), decode(0x2FBF))
  }

  #[test]
  fn skip_if_equal() {
    assert_eq!(
      Instruction::SkipIfEqual(RegisterValuePair {
        register: 0xA,
        value: 0xBB
      }),
      decode(0x3ABB)
    )
  }

  #[test]
  fn skip_if_different() {
    assert_eq!(
      Instruction::SkipIfDifferent(RegisterValuePair {
        register: 0xA,
        value: 0xBB
      }),
      decode(0x4ABB)
    )
  }

  #[test]
  fn skip_if_register_equal() {
    assert_eq!(
      Instruction::SkipIfRegisterEqual(TargetSourcePair {
        target: 0x1,
        source: 0x2
      }),
      decode(0x5120)
    )
  }

  #[test]
  fn assign_value_to_register() {
    assert_eq!(
      Instruction::AssignValueToRegister(RegisterValuePair {
        register: 0x2,
        value: 0x44
      }),
      decode(0x6244)
    )
  }

  #[test]
  fn add_value_to_register() {
    assert_eq!(
      Instruction::AddValueToRegister(RegisterValuePair {
        register: 0x3,
        value: 0x1A
      }),
      decode(0x731A)
    )
  }

  #[test]
  fn set_vx_as_vy() {
    assert_eq!(
      Instruction::AssignVYToVX(TargetSourcePair {
        target: 0x1,
        source: 0x6
      }),
      decode(0x8160)
    )
  }

  #[test]
  fn set_x_as_x_or_y() {
    assert_eq!(
      Instruction::SetXOrY(TargetSourcePair {
        target: 0x3,
        source: 0x1
      }),
      decode(0x8311)
    )
  }

  #[test]
  fn set_x_as_x_and_y() {
    assert_eq!(
      Instruction::SetXAndY(TargetSourcePair {
        target: 0x1,
        source: 0xE
      }),
      decode(0x81E2)
    )
  }

  #[test]
  fn set_x_as_x_xor_y() {
    assert_eq!(
      Instruction::SetXXorY(TargetSourcePair {
        target: 0xC,
        source: 0xA
      }),
      decode(0x8CA3)
    )
  }

  #[test]
  fn add_vy_to_vx() {
    assert_eq!(
      Instruction::AddYToX(TargetSourcePair {
        target: 0xE,
        source: 0xD
      }),
      decode(0x8ED4)
    )
  }

  #[test]
  fn subtract_vy_from_vx() {
    assert_eq!(
      Instruction::SubYFromX(TargetSourcePair {
        target: 0xE,
        source: 0xD
      }),
      decode(0x8ED5)
    )
  }

  #[test]
  fn shift_right() {
    assert_eq!(Instruction::ShiftRight(0x2), decode(0x82A6))
  }

  #[test]
  fn set_vx_as_vy_minus_vx() {
    assert_eq!(
      Instruction::SetXAsYMinusX(TargetSourcePair {
        target: 0x3,
        source: 0x2
      }),
      decode(0x8327)
    )
  }

  #[test]
  fn shift_left() {
    assert_eq!(Instruction::ShiftLeft(0xE), decode(0x8EAE))
  }

  #[test]
  fn skip_if_register_different() {
    assert_eq!(
      Instruction::SkipIfRegisterDifferent(TargetSourcePair {
        target: 0x4,
        source: 0x3
      }),
      decode(0x9430)
    )
  }

  #[test]
  fn set_i_as_n() {
    assert_eq!(Instruction::SetIAs(0x0EEE), decode(0xAEEE))
  }

  #[test]
  fn go_to_n_plus_v0() {
    assert_eq!(Instruction::GoToNPlusV0(0xABF), decode(0xBABF))
  }

  #[test]
  fn random() {
    assert_eq!(
      Instruction::Random(RegisterValuePair {
        register: 0xA,
        value: 0xBF
      }),
      decode(0xCABF)
    )
  }

  #[test]
  fn draw() {
    assert_eq!(
      Instruction::Draw {
        x: 0xA,
        y: 0xB,
        height: 0x4
      },
      decode(0xDAB4)
    )
  }

  #[test]
  fn skip_if_key_pressed() {
    assert_eq!(Instruction::SkipIfKeyPressed(0xA), decode(0xEA9E))
  }

  #[test]
  fn skip_if_key_not_pressed() {
    assert_eq!(Instruction::SkipIfKeyNotPressed(0xD), decode(0xEDA1))
  }

  #[test]
  fn get_delay() {
    assert_eq!(Instruction::SetXAsDelay(0x7), decode(0xF707))
  }

  #[test]
  fn wait_for_input_and_store_in() {
    assert_eq!(Instruction::WaitForInputAndStoreIn(0x6), decode(0xF60A))
  }

  #[test]
  fn set_delay() {
    assert_eq!(Instruction::SetDelayAsX(0x2), decode(0xF215))
  }

  #[test]
  fn set_sound() {
    assert_eq!(Instruction::SetSoundAsX(0xD), decode(0xFD18))
  }

  #[test]
  fn add_vx_to_i() {
    assert_eq!(Instruction::AddXToI(0xA), decode(0xFA1E))
  }

  #[test]
  fn assign_font_sprite_to_i() {
    assert_eq!(Instruction::SetIAsFontSprite(0x3), decode(0xF329))
  }

  #[test]
  fn set_bcd() {
    assert_eq!(Instruction::StoreBCD(0xA), decode(0xFA33))
  }

  #[test]
  fn reg_dump() {
    assert_eq!(Instruction::DumpRegisters(0xE), decode(0xFE55))
  }

  #[test]
  fn reg_load() {
    assert_eq!(Instruction::LoadRegisters(0xB), decode(0xFB65))
  }

  #[test]
  fn invalid_instruction() {
    assert_eq!(Instruction::InvalidInstruction, decode(0x5AB4))
  }
}
