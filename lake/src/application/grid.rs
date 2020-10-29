use palmer::display::{Pixels, SCREEN_WIDTH};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub pixels: Pixels,
  pub should_render: bool,
}

pub struct Grid {
  node_ref: NodeRef,
  canvas_context: Option<CanvasRenderingContext2d>,
}

impl Component for Grid {
  type Message = ();
  type Properties = Props;

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Grid {
      node_ref: NodeRef::default(),
      canvas_context: None,
    }
  }

  fn rendered(&mut self, _first_render: bool) {
    let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
    let canvas_context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();
    self.canvas_context = Some(canvas_context);
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if props.should_render {
      match &self.canvas_context {
        None => (),
        Some(context) => {
          props
            .pixels
            .chunks(SCREEN_WIDTH)
            .enumerate()
            .for_each(|(line, pixel_line)| {
              pixel_line
                .into_iter()
                .enumerate()
                .for_each(|(column, pixel)| {
                  let x = (column * 10) as f64;
                  let y = (line * 10) as f64;
                  let color = if *pixel { "white" } else { "black" };
                  context.set_fill_style(&(JsValue::from_str(color)));
                  context.begin_path();
                  context.rect(x, y, 10., 10.);
                  context.fill();
                })
            });
        }
      }
    }

    false
  }

  fn view(&self) -> Html {
    html! {
      <canvas ref=self.node_ref.clone() width=640 height=320 />
    }
  }
}
