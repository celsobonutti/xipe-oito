pub trait AudioDriver {
  fn new() -> Self;

  fn play_sound(&mut self);
}