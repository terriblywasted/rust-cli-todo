use faker_rand::en_us::company::Slogan;
use rand::Rng;
use std::collections::HashMap;
use std::io;

enum Action {
  Add,
  Complete,
  Display,
  Delete,
  Quit,
}

struct Todo {
  title: String,
  is_completed: bool,
}

impl Todo {
  fn new(title: String) -> Self {
    Self {
      title,
      is_completed: false,
    }
  }

  fn toggle_complete(&mut self) {
    self.is_completed = !self.is_completed;
    println!("'{}' is marked as {}", self.title, {
      match self.is_completed {
        true => "completed",
        false => "not completed",
      }
    });
  }
}

fn request_core_action() -> Action {
  let mut input_string = String::new();
  let mut is_waiting_for_input = true;
  let mut result_action: Action = Action::Quit;
  let error = "Please choose one of variants: 1..4";

  println!(
    "
  Select action:
  1. Add todo
  2. Complete/Incomplete todo
  3. Display list of todos
  4. Delete todo
  5. Quit
  "
  );

  while is_waiting_for_input {
    input_string.clear();
    io::stdin().read_line(&mut input_string).unwrap();

    let input: isize = input_string.trim().parse().expect(error);
    match input {
      1 => {
        is_waiting_for_input = false;
        result_action = Action::Add
      }
      2 => {
        is_waiting_for_input = false;
        result_action = Action::Complete
      }
      3 => {
        is_waiting_for_input = false;
        result_action = Action::Display
      }
      4 => {
        is_waiting_for_input = false;
        result_action = Action::Delete
      }
      5 => {
        is_waiting_for_input = false;
        result_action = Action::Quit
      }
      _ => println!("{}", error),
    }
  }

  result_action
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

fn print_todos(todos: &HashMap<u32, Todo>) {
  println!("Your todo list");

  for (id, todo) in todos {
    println!(
      "
    -----
    id: {}
    title: {}
    is_completed: {}
    -----
    ",
      id, todo.title, todo.is_completed
    );
  }
}

fn add_todo(mut todos: HashMap<u32, Todo>, title: String) -> HashMap<u32, Todo> {
  let todo = Todo::new(title);
  todos.insert(rand::thread_rng().gen::<u32>(), todo);

  todos
}

fn complete_todo(mut todos: HashMap<u32, Todo>) -> HashMap<u32, Todo> {
  let mut input_string = String::new();
  println!("Type todos id to complete");

  let error = "It seems that there is no todo with such id";

  input_string.clear();
  io::stdin().read_line(&mut input_string).unwrap();

  let id: u32 = input_string.trim().parse().expect(error);
  let todo = todos.get_mut(&id);
  match todo {
    Some(todo) => {
      &todo.toggle_complete();
    }
    _ => println!("{}", error),
  }

  todos
}

fn delete_todo(mut todos: HashMap<u32, Todo>) -> HashMap<u32, Todo> {
  let mut input_string = String::new();
  println!("Type todos id delete");

  let error = "It seems that there is no todo with such id";

  input_string.clear();
  io::stdin().read_line(&mut input_string).unwrap();

  let id: u32 = input_string.trim().parse().expect(error);
  let has_todo_with_id = todos.get(&id).is_some();
  match has_todo_with_id {
    true => {
      println!("Todo with id {} has been deleted", id);
      todos.remove(&id);
    }
    false => println!("{}", error),
  }

  todos
}

fn request_todo_name() -> String {
  let mut input_string = String::new();
  println!("Choose a name for your todo");

  io::stdin().read_line(&mut input_string).unwrap();

  input_string.trim().to_string()
}

fn wait_for_action(mut todos: HashMap<u32, Todo>) {
  let action = request_core_action();
  match action {
    Action::Add => {
      let new_todo_name = request_todo_name();
      todos = add_todo(todos, new_todo_name);
      wait_for_action(todos)
    }
    Action::Complete => {
      todos = complete_todo(todos);
      wait_for_action(todos)
    }
    Action::Display => {
      print_todos(&todos);
      wait_for_action(todos)
    }
    Action::Delete => {
      todos = delete_todo(todos);
      wait_for_action(todos)
    }
    Action::Quit => {
      println!("Cya")
    }
  }
}

fn main() {
  let todos = generate_todos(2);

  println!("Initial todos");
  print_todos(&todos);

  wait_for_action(todos)
}
