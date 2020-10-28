use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{AudioContext, OscillatorNode};
use yew::utils::window;

pub struct WebAudioDriver {}

impl palmer::audio::AudioDriver for WebAudioDriver {
  fn new() -> Self {
    Self {}
  }

  fn play_sound(&mut self) {
    let ctx = AudioContext::new().unwrap();

    let oscillator = ctx.create_oscillator().unwrap();

    oscillator
      .connect_with_audio_node(&ctx.destination())
      .unwrap();

    oscillator.start().unwrap();

    let closure = Closure::once(move || {
      oscillator.stop().unwrap();
    });

    let js_val = closure.as_ref();
    let js_func = js_val.unchecked_ref::<js_sys::Function>();

    let w = window();
    w.set_timeout_with_callback_and_timeout_and_arguments_0(js_func, 200).unwrap();

    closure.forget();
  }
}
