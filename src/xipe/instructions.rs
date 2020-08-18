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

pub struct TargetSourcePair {
    target: u8,
    source: u8,
}

pub struct RegisterValuePair {
    register: u8,
    value: u8,
}

pub enum Instruction {
    CallMachineCode(u16),
    ClearDisplay,
    Return,
    GoTo(u16),
    Call(u16),
    SkipIfEqual(RegisterValuePair),
    SkipIfDifferent(RegisterValuePair),
    SkipIfRegisterEqual(TargetSourcePair),
    SetRegisterValue(RegisterValuePair),
    AddToRegisterValue(RegisterValuePair),
    AssignVYToVX(TargetSourcePair),
    SetXOrY(TargetSourcePair),
    SetXAndY(TargetSourcePair),
    SetXXORY(TargetSourcePair),
    AddYToX(TargetSourcePair),
    SubYFromX(TargetSourcePair),
    ShiftRight(u8),
    SetXAsYMinusX(TargetSourcePair),
    ShiftLeft(u8),
    SkipIfRegisterDifferent(TargetSourcePair),
    SetIAs(u16),
    JumpToNPlusV0(u16),
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
    let bits: Vec<char> = (format!("{:X}", op_code)).chars().collect();
    bits_array.copy_from_slice(&bits[..]);

    match bits_array {
        ['0', '0', 'E', '0'] => Instruction::ClearDisplay,
        ['0', '0', 'E', 'E'] => Instruction::Return,
        ['1', c1, c2, c3] => Instruction::GoTo(hex_char_to_integer!(c1, c2, c3)),
        ['2', c1, c2, c3] => Instruction::Call(hex_char_to_integer!(c1, c2, c3)),
        ['3', register, c1, c2] => Instruction::SkipIfEqual(as_rv_pair(register, c1, c2)),
        ['4', register, c1, c2] => Instruction::SkipIfDifferent(as_rv_pair(register, c1, c2)),
        ['5', x, y, '0'] => Instruction::SkipIfRegisterEqual(as_ts_pair(x, y)),
        ['6', register, c1, c2] => Instruction::SetRegisterValue(as_rv_pair(register, c1, c2)),
        ['7', register, c1, c2] => Instruction::AddToRegisterValue(as_rv_pair(register, c1, c2)),
        ['8', x, y, '0'] => Instruction::AssignVYToVX(as_ts_pair(x, y)),
        ['8', x, y, '1'] => Instruction::SetXOrY(as_ts_pair(x, y)),
        ['8', x, y, '2'] => Instruction::SetXAndY(as_ts_pair(x, y)),
        ['8', x, y, '3'] => Instruction::SetXXORY(as_ts_pair(x, y)),
        ['8', x, y, '4'] => Instruction::AddYToX(as_ts_pair(x, y)),
        ['8', x, y, '5'] => Instruction::SubYFromX(as_ts_pair(x, y)),
        ['8', x, _, '6'] => Instruction::ShiftRight(hex_char_to_integer!(x)),
        ['8', x, y, '7'] => Instruction::SetXAsYMinusX(as_ts_pair(x, y)),
        ['8', x, _, 'E'] => Instruction::ShiftLeft(hex_char_to_integer!(x)),
        ['9', x, y, '0'] => Instruction::SkipIfRegisterDifferent(as_ts_pair(x, y)),
        ['A', c1, c2, c3] => Instruction::SetIAs(hex_char_to_integer!(c1, c2, c3)),
        ['B', c1, c2, c3] => Instruction::JumpToNPlusV0(hex_char_to_integer!(c1, c2, c3)),
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
