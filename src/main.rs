// use chrono::prelude::*;
use components::button::*;
use faker_rand::en_us::company::Slogan;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use yew::prelude::*;

mod components;

struct Todo {
  title: String,
  is_completed: bool,
  // created_at: DateTime<Utc>,
  // updated_at: Option<DateTime<Utc>>,
}

impl Todo {
  fn new(title: String) -> Self {
    Self {
      title,
      is_completed: false,
      // created_at: Utc::now(),
      // updated_at: None,
    }
  }

  // fn toggle_complete(&mut self) {
  //   self.updated_at = Some(Utc::now());
  //   self.is_completed = !self.is_completed;
  // }

  // fn updateTitle(&mut self, title: String) {
  //   self.updated_at = Some(Utc::now());
  //   self.title = title;
  // }
}

impl fmt::Display for Todo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.title)
  }
}

fn generate_todos(amount: i8) -> HashMap<u32, Todo> {
  let mut todos: HashMap<u32, Todo> = HashMap::new();

  for _ in 0..amount {
    let id = rand::thread_rng().gen::<u32>();
    let todo = Todo::new(rand::random::<Slogan>().to_string());
    todos.insert(id, todo);
  }
  todos
}

enum Msg {
  AddOne,
}

struct App {
  // `ComponentLink` is like a reference to a component.
  // It can be used to send messages to the component
  todos: HashMap<u32, Todo>,
  link: ComponentLink<Self>,
  value: i64,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    App {
      link,
      value: 0,
      todos: generate_todos(2),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddOne => {
        self.value += 1;
        // the value has changed so we need to
        // re-render for it to appear on the page
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    html! {
        <div>
            <Button label="Label" />
            {{ self.todos.iter().map(|t| t.1).collect::<Html>() }}
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <p>{ self.value }</p>
        </div>
    }
  }
}

fn main() {
  yew::start_app::<App>();
}
