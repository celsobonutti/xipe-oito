mod application;
use native_dialog::*;

pub fn main() -> iced::Result {
  let dialog = native_dialog::OpenSingleFile {
    dir: None,
    filter: None
  };

  let path = dialog.show().unwrap().unwrap_or_default();

  application::run(path)
}
