pub struct Input {
  pub keypad: [bool; 16],
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

  pub fn key_up(&mut self, key: usize) {
    self.keypad[key] = false;
  }

  pub fn key_down(&mut self, key: usize) {
    self.keypad[key] = true;
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
    input.key_down(5);
    assert!(input.is_pressed(5));
  }

  #[test]
  fn key_up() {
    let mut input = Input::new();
    input.key_down(7);
    input.key_up(7);
    assert!(!input.is_pressed(7))
  }
}
