use std::{fmt, str::FromStr};

use crate::numeric::random_chance;
use crate::context::constants;
use crate::context::unit_loader::UnitLoader;

use super::ability::Ability;
use super::environment::Position;
use super::traits::Positionable;


// Struct containing data about a single AI unit
pub struct Unit {
  id: u64,
  despawn: bool,
  position: Position,
  pub view_index: i64,

  pub level: u8,
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
  fn set_position(&mut self, position: Position) {
    self.position = position;
  }
}

impl Unit {
  pub fn new(id: u64, level: u8) -> Unit {
    return Unit {
      id,
      despawn: false,
      position: Position::FAR,
      view_index: 0,
      level,
    }
  }

  pub fn abilities(&self) -> Vec<Ability> {
    match self.id {
      _ => vec![Ability::NOTHING],
    }
  }

  pub fn despawn(&self) -> bool {
    return self.despawn;
  }

  // Return whether unit attacks player or not
  pub fn possible_move(&mut self, time: f64) -> bool {
    let mut attack_player = false;
    let chance_moved = time * constants::UNIT_MOVE_CHANCE;
    if random_chance(1.0 - chance_moved) {
      return attack_player;
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
          attack_player = true;
        }
      },
    }
    self.position = new_position;
    return attack_player;
  }

  pub fn view_short(&self, loader: &UnitLoader) -> String {
    let data = loader.get_data(self.id);
    return format!("{} ({})", data.name, self.level);
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
