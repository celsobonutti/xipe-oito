use iced::time;
use iced::{
  executor,
  keyboard::{
    Event::{KeyPressed, KeyReleased},
    KeyCode,
  },
  window, Application, Color, Column, Command, Container, Element, Settings, Subscription,
};
use palmer::Chip8;
use palmer::audio::AudioDriver;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod grid;

use grid::Grid;

struct TAD {
  pub is_playing: bool
}

impl AudioDriver for TAD {
  fn new() -> Self {
      Self {
        is_playing: false
      }
  }

  fn play_sound(&mut self) {
      self.is_playing = true;
      println!("Boop!");
  }

  fn pause_sound(&mut self) {
      if self.is_playing {
        println!("Turn off the sound!");
      }
      self.is_playing = false;
  }
}

struct Emerson {
  engine: palmer::Chip8<TAD>,
  display: grid::Grid,
  cartridge_loaded: bool,
}

fn get_key(key: KeyCode) -> Option<usize> {
  match key {
    KeyCode::Key1 => Some(0x1),
    KeyCode::Key2 => Some(0x2),
    KeyCode::Key3 => Some(0x3),
    KeyCode::Key4 => Some(0xC),
    KeyCode::Q => Some(0x4),
    KeyCode::W => Some(0x5),
    KeyCode::E => Some(0x6),
    KeyCode::R => Some(0xD),
    KeyCode::A => Some(0x7),
    KeyCode::S => Some(0x8),
    KeyCode::D => Some(0x9),
    KeyCode::F => Some(0xE),
    KeyCode::Z => Some(0xA),
    KeyCode::X => Some(0x0),
    KeyCode::C => Some(0xB),
    KeyCode::V => Some(0xF),
    _ => None,
  }
}

#[derive(Debug, Clone)]
enum Message {
  Tick(Instant),
  Display(grid::Message),
  Event(iced_native::Event),
}

#[derive(Default, Debug)]
struct Flags {
  game_path: PathBuf,
}

pub fn run(game_path: PathBuf) -> iced::Result {
  Emerson::run(Settings {
    antialiasing: false,
    flags: {
      Flags {
        game_path: game_path,
      }
    },
    window: window::Settings {
      size: (
        (palmer::display::SCREEN_WIDTH * 10) as u32,
        (palmer::display::SCREEN_HEIGHT * 10) as u32,
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
  type Flags = Flags;

  fn scale_factor(&self) -> f64 {
    10.
  }

  fn background_color(&self) -> Color {
    Color::BLACK
  }

  fn new(flags: Flags) -> (Self, Command<Message>) {
    let mut xipe = Chip8::new(TAD::new());

    let mut file = File::open(flags.game_path).unwrap();
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
        if self.engine.should_draw() {
          self
            .display
            .update(grid::Message::Show(self.engine.display.pixels));
        }
      }
      Message::Display(_) => (),
      Message::Event(event) => match event {
        iced_native::Event::Keyboard(event) => match event {
          KeyPressed {
            key_code,
            modifiers: _,
          } => {
            if let Some(key) = get_key(key_code) {
              self.engine.input.key_down(key)
            }
          }
          KeyReleased {
            key_code,
            modifiers: _,
          } => {
            if let Some(key) = get_key(key_code) {
              self.engine.input.key_up(key)
            }
          }
          _ => (),
        },
        _ => (),
      },
    }

    Command::none()
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    if self.cartridge_loaded {
      Subscription::batch(vec![
        time::every(Duration::from_millis(2 as u64)).map(Message::Tick),
        iced_native::subscription::events().map(Message::Event),
      ])
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
