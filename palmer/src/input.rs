#[derive(Clone, Copy, PartialEq)]
pub struct Input {
  pub keypad: [bool; 16],
}

#[derive(Clone, Copy, PartialEq)]
pub enum Button {
  One = 0x1,
  Two = 0x2,
  Three = 0x3,
  Four = 0x4,
  Five = 0x5,
  Six = 0x6,
  Seven = 0x7,
  Eight = 0x8,
  Nine = 0x9,
  Zero = 0x0,
  A = 0xA,
  B = 0xB,
  C = 0xC,
  D = 0xD,
  E = 0xE,
  F = 0xF
}

pub const BUTTON_LIST: [Button; 16] = [
  Button::One,
  Button::Two,
  Button::Three,
  Button::C,
  Button::Four,
  Button::Five,
  Button::Six,
  Button::D,
  Button::Seven,
  Button::Eight,
  Button::Nine,
  Button::E,
  Button::A,
  Button::Zero,
  Button::B,
  Button::F
];

impl Button {
  pub fn to_label(&self) -> &str {
    match self {
      Button::One => "1",
      Button::Two => "2",
      Button::Three => "3",
      Button::Four => "4",
      Button::Five => "5",
      Button::Six => "6",
      Button::Seven => "7",
      Button::Eight => "8",
      Button::Nine => "9",
      Button::Zero => "0",
      Button::A => "A",
      Button::B => "B",
      Button::C => "C",
      Button::D => "D",
      Button::E => "E",
      Button::F => "F"
    }
  }
}

impl Input {
  pub fn new() -> Input {
    Input {
      keypad: [false; 16],
    }
  }

  pub fn is_pressed(&self, key: u8) -> bool {
    self.keypad[key as usize]
  }

  pub fn key_up(&mut self, key: Button) {
    self.keypad[key as usize] = false;
  }

  pub fn key_down(&mut self, key: Button) {
    self.keypad[key as usize] = true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn starts_empty() {
    let input = Input::new();

    assert!(input.keypad.iter().all(|key| { *key == false }))
  }

  #[test]
  fn is_pressed() {
    let mut input = Input::new();
    input.keypad[2] = true;

    assert!(input.is_pressed(2));
    assert!(!input.is_pressed(1));
  }

  #[test]
  fn key_down() {
    let mut input = Input::new();
    input.key_down(Button::Five);
    assert!(input.is_pressed(5));
  }

  #[test]
  fn key_up() {
    let mut input = Input::new();
    input.key_down(Button::Seven);
    input.key_up(Button::Seven);
    assert!(!input.is_pressed(7))
  }
}
