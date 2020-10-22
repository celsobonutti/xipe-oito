use iced::{
  canvas::{self, Cache, Canvas, Cursor, Geometry, Path},
  Color, Element, Point, Rectangle, Size,
};
use palmer::display::{get_pixel, Pixels, SCREEN_HEIGHT, SCREEN_WIDTH};

pub const SCALE: usize = 10;

pub struct Grid {
  display: Pixels,
  display_cache: Cache,
}

impl Default for Grid {
  fn default() -> Self {
    Grid {
      display: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
      display_cache: Cache::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Message {
  Show(Pixels),
}

impl Grid {
  pub fn new() -> Grid {
    Grid::default()
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::Show(pixels) => {
        self.display_cache.clear();
        self.display = pixels;
      }
    }
  }

  pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
    Canvas::new(self)
      .width(iced::Length::Units((SCREEN_WIDTH * SCALE) as u16))
      .height(iced::Length::Units((SCREEN_HEIGHT * SCALE) as u16))
      .into()
  }
}

impl<'a> canvas::Program<Message> for Grid {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let pixels = self.display_cache.draw(bounds.size(), |frame| {
      let background = Path::rectangle(Point::ORIGIN, frame.size());
      frame.fill(&background, Color::BLACK);

      frame.with_save(|frame| {
        for x in 0..SCREEN_WIDTH {
          for y in 0..SCREEN_HEIGHT {
            let color = if get_pixel(self.display, x, y) {
              Color::WHITE
            } else {
              Color::TRANSPARENT
            };

            frame.fill_rectangle(
              Point::new((x * SCALE) as f32, (y * SCALE) as f32),
              Size::new(SCALE as f32, SCALE as f32),
              color,
            )
          }
        }
      });
    });

    vec![pixels]
  }
}
