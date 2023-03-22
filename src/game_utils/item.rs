use std::{fmt, str::FromStr};
use rand::seq::SliceRandom;

use super::{environment::{Position, Positionable}, player::RotfPlayer};


pub struct Item {
  id: u64,
  despawn: bool,
  position: Position,
  pub view_index: i64,

  pub level: u8,
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
      despawn: false,
      position: Position::MEDIUM,
      view_index: 0,
      level: 0,
    }
  }

  pub fn spawn(player: &RotfPlayer) -> Item {
    let possible_ids = Item::possible_ids(player.tier());
    match possible_ids.choose(&mut rand::thread_rng()) {
      Some(id) => return Item::new(*id),
      None => return Item::new(0),
    }
  }

  fn possible_ids(tier: u8) -> Vec<u64> {
    match tier {
      _ => vec![],
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\n   id: {}", self.id);
    contents += &format!("\n   position: {}", self.position);
    contents += &format!("\n   view_index: {}", self.view_index);
    contents += &format!("\n   despawn: {}", self.despawn);
    contents += &format!("\n   level: {}", self.level);
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
      "level"      => self.level      = value.parse::<u8>().unwrap_or(0),
      _ => {},
    }
  }
}
