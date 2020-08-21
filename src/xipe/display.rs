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

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) {}
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
}
