use super::environment;


pub struct Unit {
  position: environment::Position,
}

impl environment::Positionable for Unit {
  fn position(&self) -> environment::Position {
    return self.position.clone();
  }
}

impl Unit {
  pub fn new() -> Unit {
    return Unit {
      position: environment::Position::MEDIUM,
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\n   position: {}", self.position);
    return contents;
  }

  pub fn read_line(&mut self, line: String) {
    let (key, mut value) = line.split_once(":").unwrap();
    value = value.trim();
    match key.trim() {
      _ => {},
    }
  }
}