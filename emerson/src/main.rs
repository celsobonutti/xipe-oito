mod application;

use native_dialog;

pub fn main() -> iced::Result {
  let dialog = native_dialog::FileDialog::new().show_open_single_file();

  let path = dialog.unwrap().unwrap_or_default();

  application::run(path)
}
