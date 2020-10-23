pub trait AudioDriver {
  fn new() -> Self;

  fn play_sound(&mut self);
  fn pause_sound(&mut self);
}