use super::environment;


pub struct Item {
  position: environment::Position,
}

impl environment::Positionable for Item {
  fn position(&self) -> environment::Position {
    return self.position.clone();
  }
}

impl Item {
  pub fn new() -> Item {
    return Item {
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