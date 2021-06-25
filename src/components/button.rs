use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub label: String,
}

pub struct Button {
  props: Props,
}

pub enum Msg {}

impl Component for Button {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Button { props }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    html! {
      <span class="rounded-md inline-block text-center text-sm bg-indigo-800 px-4 py-2 text-white hover:bg-indigo-600">
          { &self.props.label }
      </span>
    }
  }
}
