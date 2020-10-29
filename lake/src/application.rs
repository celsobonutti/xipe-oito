use palmer::audio::AudioDriver;
use palmer::input::Button;
use palmer::Chip8;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::keyboard::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::ChangeData;

mod audio;
mod button;
mod buttons;
mod grid;

use audio::WebAudioDriver;
use buttons::Buttons;
use grid::Grid;

pub struct Lake {
  link: ComponentLink<Lake>,
  pub engine: Chip8<WebAudioDriver>,
  tasks: Vec<ReaderTask>,
  is_running: bool,
  _task: IntervalTask,
  _key_up_listener: KeyListenerHandle,
  _key_down_listener: KeyListenerHandle,
}

fn parse_key(key: &str) -> Option<Button> {
  match key {
    "Digit1" => Some(Button::One),
    "Digit2" => Some(Button::Two),
    "Digit3" => Some(Button::Three),
    "Digit4" => Some(Button::C),
    "KeyQ" => Some(Button::Four),
    "KeyW" => Some(Button::Five),
    "KeyE" => Some(Button::Six),
    "KeyR" => Some(Button::D),
    "KeyA" => Some(Button::Seven),
    "KeyS" => Some(Button::Eight),
    "KeyD" => Some(Button::Nine),
    "KeyF" => Some(Button::E),
    "KeyZ" => Some(Button::A),
    "KeyX" => Some(Button::Zero),
    "KeyC" => Some(Button::B),
    "KeyV" => Some(Button::F),
    _ => None,
  }
}

pub enum Message {
  Files(Vec<File>),
  FileLoaded(FileData),
  Tick,
  KeyDownEvent(Option<Button>),
  KeyUpEvent(Option<Button>),
}

impl Component for Lake {
  type Message = Message;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let tick_callback = link.callback(|_| Message::Tick);

    let key_down_callback = link.callback(|e: KeyboardEvent| {
      let key = parse_key(&e.code());
      Message::KeyDownEvent(key)
    });
    let key_up_callback = link.callback(|e: KeyboardEvent| {
      let key = parse_key(&e.code());
      Message::KeyUpEvent(key)
    });

    let wnd = &web_sys::window().unwrap();
    let task = IntervalService::spawn(Duration::from_millis(2), tick_callback);
    let key_down_listener = KeyboardService::register_key_down(wnd, key_down_callback);
    let key_up_listener = KeyboardService::register_key_up(wnd, key_up_callback);

    let engine = Chip8::new(WebAudioDriver::new());
    Self {
      link,
      engine: engine,
      tasks: vec![],
      is_running: false,
      _task: task,
      _key_down_listener: key_down_listener,
      _key_up_listener: key_up_listener,
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
      Message::KeyDownEvent(input) => {
        if let Some(key) = input {
          self.engine.input.key_down(key)
        }
      }
      Message::KeyUpEvent(input) => {
        if let Some(key) = input {
          self.engine.input.key_up(key)
        }
      }
    }

    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let should_draw = self.engine.should_draw();
    let pixels = self.engine.display.pixels;

    html! {
      <main>
        <div class="view">
          <Grid should_render=should_draw pixels=pixels />
          <div class="game__loader">
            <input type="file" id="file" multiple=false onchange=self.link.callback(move |value| {
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
            <label for="file">{"LOAD GAME"}</label>
          </div>
          <Buttons
            onkeydown=self.link.callback(|code| {
              Message::KeyDownEvent(Some(code))
            })
            onkeyup=self.link.callback(|code| {
              Message::KeyUpEvent(Some(code))
            })
            active_buttons=self.engine.input
          />
        </div>
      </main>
    }
  }
}
