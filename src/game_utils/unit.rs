use std::{fmt, str::FromStr};

use crate::numeric::random_chance;

use super::environment::{Position, Positionable};


pub struct Unit {
  id: u64,
  despawn: bool,
  position: Position,
  pub view_index: i64,
}

impl fmt::Display for Unit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ID: {}", self.id)
  }
}

impl Positionable for Unit {
  fn position(&self) -> Position {
    return self.position.clone();
  }
}

impl Unit {
  pub fn new(id: u64) -> Unit {
    return Unit {
      id,
      despawn: false,
      position: Position::FAR,
      view_index: 0,
    }
  }

  pub fn despawn(&self) -> bool {
    return self.despawn;
  }

  pub fn possible_move(&mut self) {
    if random_chance(0.40) {
      return;
    }
    let mut new_position = self.position.clone();
    match self.position {
      Position::FAR => {
        if random_chance(0.5) {
          new_position = Position::MEDIUM;
        }
        else {
          self.despawn = true;
        }
      },
      Position::MEDIUM => {
        if random_chance(0.5) {
          new_position = Position::FAR;
        }
        else {
          new_position = Position::NEAR;
        }
      },
      Position::NEAR => {
        if random_chance(0.5) {
          new_position = Position::MEDIUM;
        }
        else {
          // try to attack player
        }
      },
    }
    self.position = new_position;
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\n   id: {}", self.id);
    contents += &format!("\n   position: {}", self.position);
    contents += &format!("\n   view_index: {}", self.view_index);
    contents += &format!("\n   despawn: {}", self.despawn);
    return contents;
  }

  pub fn read_line(&mut self, line: String) {
    let (key, mut value) = line.split_once(":").unwrap();
    value = value.trim();
    match key.trim() {
      "id"         => self.id         = value.parse::<u64>().unwrap_or(0),
      "position"   => self.position   = Position::from_str(value).unwrap_or(Position::FAR),
      "view_index" => self.view_index = value.parse::<i64>().unwrap_or(-1),
      "despawn"    => self.despawn    = value.parse::<bool>().unwrap_or(true),
      _ => {},
    }
  }
}
