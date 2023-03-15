use std::fmt;
use std::str::FromStr;
use std::io::{Error, BufRead};

use crate::context::unit_loader::UnitLoader;
use crate::filesystem;

use super::player::RotfPlayer;
use super::unit::Unit;
use super::item::Item;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;


// Relative to player in environment
#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum Position {
  NEAR,
  MEDIUM,
  FAR,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for Position {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for state in Position::iter() {
      if state.to_string() == s {
        return Ok(state);
      }
    }
    Err(())
  }
}

impl Position {
  pub fn distance(&self) -> u64 {
    match self {
      Position::NEAR => 1,
      Position::MEDIUM => 2,
      Position::FAR => 3,
    }
  }
}


// Trait for position
pub trait Positionable {
  fn position(&self) -> Position;
}


// Environment player is in
pub struct RotfEnvironment {
  pub units: Vec<Unit>,
  pub items: Vec<Item>,
}

impl RotfEnvironment {
  pub fn new() -> RotfEnvironment {
    return RotfEnvironment {
      units: vec![],
      items: vec![],
    }
  }

  pub fn update(&mut self, player: &RotfPlayer, unit_loader: &UnitLoader) {
    // allow units to move
    for unit in self.units.iter_mut() {
      unit.possible_move();
    }
    // despawn units
    self.units.retain(|u| !u.despawn());
    // respawn units
    let num_units = self.num_units(player.tier());
    while self.units.len() < num_units {
      self.units.push(Unit::new(unit_loader.spawn()));
    }
    // despawn items
    // spawn items
    let num_items = self.num_items(player.tier());
    while self.items.len() < num_items {
      self.items.push(Item::spawn(player));
    }
  }

  fn num_units(&self, tier: u8) -> usize {
    match tier {
      1 => 10,
      _ => 0,
    }
  }

  fn num_items(&self, tier: u8) -> usize {
    match tier {
      _ => 0,
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    for unit in &self.units {
      contents += "\n%%% BEGIN UNIT";
      contents += &unit.file_content();
      contents += "\n%%% END UNIT\n";
    }
    for item in &self.items {
      contents += "\n%%% BEGIN ITEM";
      contents += &item.file_content();
      contents += "\n%%% END ITEM\n";
    }
    return contents;
  }

  pub fn load(&mut self, save_name: String) -> Result<(), Error> {
    let mut in_unit = false;
    let mut in_item = false;
    let mut curr_unit = Unit::new(0);
    let mut curr_item = Item::new(0);
    for oline in filesystem::open_file(format!("data/saves/{}/environment.rotf", save_name))?.lines() {
      let line = oline?;
      if !line.clone().contains(":") {
        continue;
      }
      match line.trim() {
        "%%% BEGIN UNIT" => {
          in_unit = true;
        }
        "%%% END UNIT" => {
          in_unit = false;
          self.units.push(curr_unit);
          curr_unit = Unit::new(0);
        }
        "%%% BEGIN ITEM" => {
          in_item = true;
        }
        "%%% END ITEM" => {
          in_item = false;
          self.items.push(curr_item);
          curr_item = Item::new(0);
        }
        _ => {},
      }
      // check for unit or item
      if in_unit {
        curr_unit.read_line(line);
        continue;
      }
      if in_item {
        curr_item.read_line(line);
        continue;
      }
      // environment data
      let (key, mut _value) = line.split_once(":").unwrap();
      _value = _value.trim();
      match key.trim() {
        _ => {},
      }
    }
    Ok(())
  }
}
