use iced::time;
use iced::{
  executor,
  keyboard::{
    Event::{KeyPressed, KeyReleased},
    KeyCode,
  },
  window, Application, Color, Column, Command, Container, Element, Settings, Subscription,
};
use palmer::audio::AudioDriver;
use palmer::Chip8;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod audio;
mod grid;

use audio::NativeAudioDriver;
use grid::Grid;

struct Emerson {
  engine: palmer::Chip8<NativeAudioDriver>,
  display: grid::Grid,
  cartridge_loaded: bool,
}

fn parse_key(key: KeyCode) -> Option<usize> {
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
    let mut xipe = Chip8::new(NativeAudioDriver::new());

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
        let mut closure = || {
          self.engine.emulate_cycle();
          if self.engine.should_draw() {
            self
              .display
              .update(grid::Message::Show(self.engine.display.pixels));
          }
        };
        if cfg!(target_os = "windows") {
          for _ in 0..5 {
            closure();
          }
        } else {
          closure();
        }
      }
      Message::Display(_) => (),
      Message::Event(event) => match event {
        iced_native::Event::Keyboard(event) => match event {
          KeyPressed {
            key_code,
            modifiers: _,
          } => {
            if let Some(key) = parse_key(key_code) {
              self.engine.input.key_down(key)
            }
          }
          KeyReleased {
            key_code,
            modifiers: _,
          } => {
            if let Some(key) = parse_key(key_code) {
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
    let time = if cfg!(target_os = "windows") { 10 } else { 2 };

    if self.cartridge_loaded {
      Subscription::batch(vec![
        time::every(Duration::from_millis(time as u64)).map(Message::Tick),
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
