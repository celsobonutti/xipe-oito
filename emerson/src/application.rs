use iced::time;
use iced::{
  executor, window, Application, Column, Command, Container, Element, Settings, Subscription,
};
use palmer::Chip8;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod grid;

use grid::Grid;

struct Emerson {
  engine: palmer::Chip8,
  display: grid::Grid,
  cartridge_loaded: bool,
}

#[derive(Debug, Clone)]
enum Message {
  Tick(Instant),
  Display(grid::Message),
}

pub fn run() -> iced::Result {
  Emerson::run(Settings {
    antialiasing: false,
    window: window::Settings {
      size: (
        (palmer::display::SCREEN_WIDTH * grid::SCALE) as u32,
        (palmer::display::SCREEN_HEIGHT * grid::SCALE) as u32,
      ),
      resizable: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

impl Application for Emerson {
  type Message = Message;
  type Executor = executor::Default;
  type Flags = ();

  fn new(_flags: ()) -> (Self, Command<Message>) {
    let mut xipe = Chip8::new(Box::new(|| {}));

    let mut file = File::open("particle.ch8").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    xipe.load(buffer);

    (
      Self {
        engine: xipe,
        display: Grid::new(),
        cartridge_loaded: true,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Xipe Oito!")
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::Tick(_) => {
        self.engine.emulate_cycle();
        self
          .display
          .update(grid::Message::Show(self.engine.display.pixels))
      }
      Message::Display(_) => (),
    }

    Command::none()
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    if self.cartridge_loaded {
      time::every(Duration::from_millis(2 as u64)).map(Message::Tick)
    } else {
      Subscription::none()
    }
  }

  fn view(&mut self) -> Element<Message> {
    let content = Column::new().push(
      self
        .display
        .view()
        .map(move |message| Message::Display(message)),
    );

    Container::new(content).into()
  }
}
