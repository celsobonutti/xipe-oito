#![recursion_limit="1024"]

mod application;

use application::Lake;

fn main() {
  yew::start_app::<Lake>();
}
