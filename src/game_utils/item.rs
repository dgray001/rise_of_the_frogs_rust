use std::{fmt, str::FromStr};

use super::environment::{Position, Positionable};


pub struct Item {
  id: u64,
  position: Position,
  pub view_index: i64,
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ID: {}", self.id)
  }
}

impl Positionable for Item {
  fn position(&self) -> Position {
    return self.position.clone();
  }
}

impl Item {
  pub fn new(id: u64) -> Item {
    return Item {
      id,
      position: Position::MEDIUM,
      view_index: 0,
    }
  }
  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\n   id: {}", self.id);
    contents += &format!("\n   position: {}", self.position);
    contents += &format!("\n   view_index: {}", self.view_index);
    return contents;
  }

  pub fn read_line(&mut self, line: String) {
    let (key, mut value) = line.split_once(":").unwrap();
    value = value.trim();
    match key.trim() {
      "id" => self.id = value.parse::<u64>().unwrap_or(0),
      "position" => self.position = Position::from_str(value).unwrap_or(Position::FAR),
      "view_index" => self.view_index = value.parse::<i64>().unwrap_or(-1),
      _ => {},
    }
  }
}
