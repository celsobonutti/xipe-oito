use palmer::Chip8;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::services::{ConsoleService, keyboard::*};
use yew::ChangeData;

mod application;

use palmer::audio::AudioDriver;
use application::grid::Grid;
use application::audio::WebAudioDriver;

pub struct Lake {
  link: ComponentLink<Lake>,
  pub engine: Chip8<WebAudioDriver>,
  tasks: Vec<ReaderTask>,
  is_running: bool,
  _task: IntervalTask,
  _key_up_listener: KeyListenerHandle,
  _key_down_listener: KeyListenerHandle
}

fn parse_key(key: &str) -> Option<usize> {
  match key {
    "Digit1" => Some(0x1),
    "Digit2" => Some(0x2),
    "Digit3" => Some(0x3),
    "Digit4" => Some(0xC),
    "KeyQ" => Some(0x4),
    "KeyW" => Some(0x5),
    "KeyE" => Some(0x6),
    "KeyR" => Some(0xD),
    "KeyA" => Some(0x7),
    "KeyS" => Some(0x8),
    "KeyD" => Some(0x9),
    "KeyF" => Some(0xE),
    "KeyZ" => Some(0xA),
    "KeyX" => Some(0x0),
    "KeyC" => Some(0xB),
    "KeyV" => Some(0xF),
    _ => None,
  }
}

pub enum Message {
  Files(Vec<File>),
  FileLoaded(FileData),
  Tick,
  KeyDownEvent(KeyboardEvent),
  KeyUpEvent(KeyboardEvent)
}

impl Component for Lake {
  type Message = Message;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(|_| Message::Tick);
    let task = IntervalService::spawn(Duration::from_millis(2), callback);
    let key_down_listener = KeyboardService::register_key_down(&web_sys::window().unwrap(), (&link).callback(|e: KeyboardEvent| Message::KeyDownEvent(e)));
    let key_up_listener = KeyboardService::register_key_up(&web_sys::window().unwrap(), (&link).callback(|e: KeyboardEvent| Message::KeyUpEvent(e)));

    let engine = Chip8::new(WebAudioDriver::new());
    Self {
      link,
      engine: engine,
      tasks: vec![],
      is_running: false,
      _task: task,
      _key_down_listener: key_down_listener,
      _key_up_listener: key_up_listener
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Message::Files(files) => {
        let file = files.first().unwrap().clone();
        let callback = self.link.callback(Message::FileLoaded);
        let task = ReaderService::default().read_file(file, callback).unwrap();
        self.tasks.push(task);
      }
      Message::FileLoaded(file) => {
        self.is_running = false;
        self.engine.reset();
        self.engine.load(file.content);
        self.is_running = true;
      }
      Message::Tick => {
        if self.is_running {
          self.engine.emulate_cycle();
        }
      }
      Message::KeyDownEvent(keyboard_event) => {
        if let Some(key) = parse_key(keyboard_event.code().as_str()) {
          self.engine.input.key_down(key)
        }
      }
      Message::KeyUpEvent(keyboard_event) => {
        if let Some(key) = parse_key(keyboard_event.code().as_str()) {
          self.engine.input.key_up(key)
        }
      }
    }

    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn rendered(&mut self, _first_render: bool) {
      
  }

  fn view(&self) -> Html {
    let should_draw = self.engine.should_draw();
    let pixels = self.engine.display.pixels;

    html! {
      <main>
        <h1 >{ "Chip8" }</h1>
        <input type="file" multiple=false onchange=self.link.callback(move |value| {
          let mut result = Vec::new();
          if let ChangeData::Files(files) = value {
              let files = js_sys::try_iter(&files)
                  .unwrap()
                  .unwrap()
                  .map(|v| File::from(v.unwrap()));
              result.extend(files);
          }
          Message::Files(result)
          })
         />
        <Grid should_render=should_draw pixels=pixels />
      </main>
    }
  }
}

fn main() {
  yew::start_app::<Lake>();
}
