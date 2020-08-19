macro_rules! hex_char_to_integer {
    ( $char1:expr ) => {
        u8::from_str_radix(&$char1.to_string(), 16).unwrap()
    };
    ( $char1:expr, $char2:expr ) => {{
        let string = format!("{}{}", $char1, $char2);
        u8::from_str_radix(&string, 16).unwrap()
    }};
    ( $char1:expr, $char2:expr, $char3:expr, $char4:expr ) => {{
        let string = format!("{}{}{}{}", $char1, $char2, $char3, $char4);
        u16::from_str_radix(&string, 16).unwrap()
    }};
    ( $char1:expr, $char2:expr, $char3:expr ) => {{
        hex_char_to_integer!('0', $char1, $char2, $char3)
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
    SetXAsKey(u8),
    SetDelayAsX(u8),
    SetSoundAsX(u8),
    AddXToI(u8),
    SetIAsSprite(u8),
    StoreBCD(u8),
    DumpRegisters(u8),
    LoadRegisters(u8),
    InvalidInstruction,
}

fn as_ts_pair(x: char, y: char) -> TargetSourcePair {
    TargetSourcePair {
        target: hex_char_to_integer!(x),
        source: hex_char_to_integer!(y),
    }
}

fn as_rv_pair(register: char, c1: char, c2: char) -> RegisterValuePair {
    RegisterValuePair {
        register: hex_char_to_integer!(register),
        value: hex_char_to_integer!(c1, c2),
    }
}

pub fn decode(op_code: u16) -> Instruction {
    let mut bits_array = [' '; 4];
    let bits: Vec<char> = (format!("{:04X}", op_code)).chars().collect();
    bits_array.copy_from_slice(&bits[..]);

    match bits_array {
        ['0', '0', 'E', '0'] => Instruction::ClearDisplay,
        ['0', '0', 'E', 'E'] => Instruction::Return,
        ['0', c1, c2, c3] => Instruction::CallMachineCode(hex_char_to_integer!(c1, c2, c3)),
        ['1', c1, c2, c3] => Instruction::GoTo(hex_char_to_integer!(c1, c2, c3)),
        ['2', c1, c2, c3] => Instruction::Call(hex_char_to_integer!(c1, c2, c3)),
        ['3', register, c1, c2] => Instruction::SkipIfEqual(as_rv_pair(register, c1, c2)),
        ['4', register, c1, c2] => Instruction::SkipIfDifferent(as_rv_pair(register, c1, c2)),
        ['5', x, y, '0'] => Instruction::SkipIfRegisterEqual(as_ts_pair(x, y)),
        ['6', register, c1, c2] => Instruction::AssignValueToRegister(as_rv_pair(register, c1, c2)),
        ['7', register, c1, c2] => Instruction::AddValueToRegister(as_rv_pair(register, c1, c2)),
        ['8', x, y, '0'] => Instruction::AssignVYToVX(as_ts_pair(x, y)),
        ['8', x, y, '1'] => Instruction::SetXOrY(as_ts_pair(x, y)),
        ['8', x, y, '2'] => Instruction::SetXAndY(as_ts_pair(x, y)),
        ['8', x, y, '3'] => Instruction::SetXXorY(as_ts_pair(x, y)),
        ['8', x, y, '4'] => Instruction::AddYToX(as_ts_pair(x, y)),
        ['8', x, y, '5'] => Instruction::SubYFromX(as_ts_pair(x, y)),
        ['8', x, _, '6'] => Instruction::ShiftRight(hex_char_to_integer!(x)),
        ['8', x, y, '7'] => Instruction::SetXAsYMinusX(as_ts_pair(x, y)),
        ['8', x, _, 'E'] => Instruction::ShiftLeft(hex_char_to_integer!(x)),
        ['9', x, y, '0'] => Instruction::SkipIfRegisterDifferent(as_ts_pair(x, y)),
        ['A', c1, c2, c3] => Instruction::SetIAs(hex_char_to_integer!(c1, c2, c3)),
        ['B', c1, c2, c3] => Instruction::GoToNPlusV0(hex_char_to_integer!(c1, c2, c3)),
        ['C', register, c1, c2] => Instruction::Random(as_rv_pair(register, c1, c2)),
        ['D', x, y, height] => Instruction::Draw {
            x: hex_char_to_integer!(x),
            y: hex_char_to_integer!(y),
            height: hex_char_to_integer!(height),
        },
        ['E', x, '9', 'E'] => Instruction::SkipIfKeyPressed(hex_char_to_integer!(x)),
        ['E', x, 'A', '1'] => Instruction::SkipIfKeyNotPressed(hex_char_to_integer!(x)),
        ['F', x, '0', '7'] => Instruction::SetXAsDelay(hex_char_to_integer!(x)),
        ['F', x, '0', 'A'] => Instruction::SetXAsKey(hex_char_to_integer!(x)),
        ['F', x, '1', '5'] => Instruction::SetDelayAsX(hex_char_to_integer!(x)),
        ['F', x, '1', '8'] => Instruction::SetSoundAsX(hex_char_to_integer!(x)),
        ['F', x, '1', 'E'] => Instruction::AddXToI(hex_char_to_integer!(x)),
        ['F', x, '2', '9'] => Instruction::SetIAsSprite(hex_char_to_integer!(x)),
        ['F', x, '3', '3'] => Instruction::StoreBCD(hex_char_to_integer!(x)),
        ['F', x, '5', '5'] => Instruction::DumpRegisters(hex_char_to_integer!(x)),
        ['F', x, '6', '5'] => Instruction::LoadRegisters(hex_char_to_integer!(x)),
        _ => Instruction::InvalidInstruction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn get_key() {
        assert_eq!(Instruction::SetXAsKey(0x6), decode(0xF60A))
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
    fn set_sprite_to_i() {
        assert_eq!(Instruction::SetIAsSprite(0x3), decode(0xF329))
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
