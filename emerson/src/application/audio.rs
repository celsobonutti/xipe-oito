use palmer::audio::AudioDriver;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub struct NativeAudioDriver {
  sender: Sender<Message>,
}

enum Message {
  Play,
  Stop,
}

impl AudioDriver for NativeAudioDriver {
  fn new() -> Self {
    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || {
      let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
      let sink = rodio::Sink::try_new(&stream_handle).unwrap();

      let source = rodio::source::SineWave::new(440);
      sink.append(source);
      sink.pause();

      for received in rx {
        match received {
          Message::Play => {
            sink.play();
            println!("Caralho marreco!");
            thread::sleep(std::time::Duration::from_millis(200));
            sink.pause();
          },
          Message::Stop => {
            ()
          },
        }
      }
    });

    Self { sender: tx }
  }

  fn play_sound(&mut self) {
    self.sender.send(Message::Play).unwrap();
  }

  fn pause_sound(&mut self) {
    self.sender.send(Message::Stop).unwrap();
  }
}
