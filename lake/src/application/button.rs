use palmer::input::Button as InputButton;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub text: &'static str,
  pub onmousedown: Callback<InputButton>,
  pub onmouseup: Callback<InputButton>,
  pub code: InputButton,
  pub is_active: bool,
}

pub enum Message {
  MouseDown,
  MouseUp,
}

pub struct Button {
  text: &'static str,
  onmousedown: Callback<InputButton>,
  onmouseup: Callback<InputButton>,
  code: InputButton,
  link: ComponentLink<Self>,
  is_active: bool,
}

impl Component for Button {
  type Message = Message;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      text: props.text,
      onmousedown: props.onmousedown,
      onmouseup: props.onmouseup,
      code: props.code,
      link: link,
      is_active: props.is_active,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Message::MouseDown => self.onmousedown.emit(self.code),
      Message::MouseUp => self.onmouseup.emit(self.code),
    }

    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let mut should_render = false;
    if self.text != props.text {
      should_render = true;
      self.text = props.text;
    }
    if self.is_active != props.is_active {
      should_render = true;
      self.is_active = props.is_active;
    }
    should_render
  }

  fn view(&self) -> Html {
    html! {
      <button
        class=format!("buttons__button {}", if self.is_active { "buttons__button--active" } else { "" })
        onmouseup=self.link.callback(|_| { Message::MouseUp })
        onmousedown=self.link.callback(|_| { Message::MouseDown })
        ontouchstart=self.link.callback(|_| { Message::MouseUp })
        ontouchend=self.link.callback(|_| { Message::MouseDown })
      >{self.text}</button>
    }
  }
}
