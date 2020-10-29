use palmer::input::{Button as InputButton, BUTTON_LIST, Input};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use super::button::Button;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub onkeyup: Callback<InputButton>,
  pub onkeydown: Callback<InputButton>,
  pub active_buttons: Input
}

pub enum Message {
  KeyUp(InputButton),
  KeyDown(InputButton),
}

pub struct Buttons {
  onkeyup: Callback<InputButton>,
  onkeydown: Callback<InputButton>,
  active_buttons: Input,
  link: ComponentLink<Self>,
}

impl Component for Buttons {
  type Message = Message;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      onkeydown: props.onkeydown,
      onkeyup: props.onkeyup,
      link: link,
      active_buttons: props.active_buttons
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Message::KeyUp(key) => self.onkeyup.emit(key),
      Message::KeyDown(key) => self.onkeydown.emit(key),
    }

    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.active_buttons != props.active_buttons {
      self.active_buttons = props.active_buttons;
      return true;
    }

    false
  }

  fn view(&self) -> Html {
    let buttons: Html = BUTTON_LIST
      .iter()
      .map(|button_code| {
        html! {
          <Button
            onmouseup=self.link.callback(|code| {Message::KeyUp(code)})
            onmousedown=self.link.callback(|code| {Message::KeyDown(code)})
            code=button_code
            text=button_code.to_label()
            is_active=self.active_buttons.is_pressed(*button_code as u8)
          />
        }
      })
      .collect();

    html! {
      <div class="buttons">
        {buttons}
      </div>
    }
  }
}
