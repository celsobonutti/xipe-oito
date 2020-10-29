use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub text: &'static str
}

pub struct Button {
  text: &'static str
}

impl Component for Button {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      text: props.text
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.text = props.text;
    
    true
  }

  fn view(&self) -> Html {
    html! {
      <button>{self.text}</button>
    }
  }
}