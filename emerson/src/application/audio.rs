use palmer::audio::AudioDriver;
use rust_embed::RustEmbed;
use rodio::Sink;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct NativeAudioDriver {
  sink: Sink
}

impl AudioDriver for NativeAudioDriver {
  fn new() -> Self {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = rodio::source::SineWave::new(440);
    sink.append(source);
    
    NativeAudioDriver {
      sink: sink
    }
  }

  fn play_sound(&mut self) {
      self.sink.play()
  }

  fn pause_sound(&mut self) {
      self.sink.stop()
  }
}