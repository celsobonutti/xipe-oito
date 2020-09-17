const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Display {
  pixels: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
  pub fn new() -> Display {
    Display {
      pixels: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
    }
  }

  pub fn clear(&mut self) {
    self.pixels = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
  }

  fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
    self.pixels[x + y * SCREEN_WIDTH] = value;
  }

  fn get_pixel(&self, x: usize, y: usize) -> bool {
    self.pixels[x + y * SCREEN_WIDTH]
  }

  fn xor_pixel(&mut self, x: usize, y: usize, new_value: bool) {
    let current = self.get_pixel(x, y);
    self.set_pixel(x, y, current ^ new_value);
  }

  pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) {
    sprite.iter().enumerate().for_each(|(line_number, line)| {
      if line_number + y < SCREEN_HEIGHT {
        format!("{:08b}", line)
          .chars()
          .map(|char| char == '1')
          .enumerate()
          .for_each(|(column_number, pixel)| {
            if column_number + x < SCREEN_WIDTH {
              self.xor_pixel(x + column_number, y + line_number, pixel);
            }
          });
      }
    });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_pixel() {
    let mut display = Display::new();
    display.pixels[1] = true;
    assert_eq!(display.get_pixel(1, 0), true);
    assert_eq!(display.get_pixel(0, 0), false);
  }

  #[test]
  fn set_pixel() {
    let mut display = Display::new();
    display.set_pixel(1, 1, true);
    assert_eq!(display.get_pixel(1, 1), true);
    display.set_pixel(1, 1, false);
    assert_eq!(display.get_pixel(1, 1), false);
  }

  #[test]
  fn clear_screen() {
    let mut display = Display::new();
    display.set_pixel(1, 3, true);
    display.set_pixel(5, 15, true);
    display.clear();
    assert!(display.pixels.iter().all(|pixel| { !*pixel }));
  }

  #[test]
  fn draw() {
    let lines: [u8; 4] = [0b01101100, 0b00011000, 0b00011000, 0b00111100];
    let mut display = Display::new();
    display.draw(0, 0, &lines);
    for i in 0..4 {
      for j in 0..8 {
        if i == 0 {
          assert_eq!(display.get_pixel(j, i), [1, 2, 4, 5].contains(&j));
        } else if i < 3 {
          assert_eq!(display.get_pixel(j, i), [3, 4].contains(&j));
        } else {
          assert_eq!(display.get_pixel(j, i), [2, 3, 4, 5].contains(&j));
        }
      }
    }
  }

  #[test]
  fn draw_erases() {
    let lines: [u8; 4] = [0b01101100, 0b00011000, 0b00011000, 0b00111100];
    let mut display = Display::new();
    display.draw(0, 0, &lines);
    display.draw(0, 0, &lines);
    assert!(display.pixels.iter().all(|pixel| { !*pixel }));
  }
}
