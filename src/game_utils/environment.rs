use std::fmt;
use std::str::FromStr;
use std::io::{Error, BufRead};

use crate::filesystem;

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


// Trait for position
pub trait Positionable {
  fn position(&self) -> Position;
}


// Environment player is in
pub struct RotfEnvironment {
  units: Vec<Unit>,
  items: Vec<Item>,
}

impl RotfEnvironment {
  pub fn new() -> RotfEnvironment {
    return RotfEnvironment {
      units: vec![],
      items: vec![],
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    for unit in &self.units {
      contents += "%%% BEGIN UNIT";
      contents += &unit.file_content();
      contents += "%%% END UNIT";
    }
    for item in &self.items {
      contents += "%%% BEGIN ITEM";
      contents += &item.file_content();
      contents += "%%% END ITEM";
    }
    return contents;
  }

  pub fn load(&mut self, save_name: String) -> Result<(), Error> {
    let mut in_unit = false;
    let mut in_item = false;
    let mut curr_unit = Unit::new();
    let mut curr_item = Item::new();
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
          curr_unit = Unit::new();
        }
        "%%% BEGIN ITEM" => {
          in_item = true;
        }
        "%%% END ITEM" => {
          in_item = false;
          self.items.push(curr_item);
          curr_item = Item::new();
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
      let (key, mut value) = line.split_once(":").unwrap();
      value = value.trim();
      match key.trim() {
        _ => {},
      }
    }
    Ok(())
  }
}