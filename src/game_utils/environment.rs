use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::io::{Error, BufRead};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::context::item_loader::ItemLoader;
use crate::context::unit_loader::UnitLoader;
use crate::filesystem;

use super::player::RotfPlayer;
use super::unit::Unit;
use super::item::Item;
use super::traits::Positionable;


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


// Environment player is in
pub struct RotfEnvironment {
  pub units: HashMap<u64, Unit>,
  pub next_unit_key: u64, // will never repeat keys
  pub items: HashMap<u64, Item>,
  pub next_item_key: u64, // will never repeat keys

  time_passed: u8, // time that needs to pass
}

impl RotfEnvironment {
  pub fn new() -> RotfEnvironment {
    return RotfEnvironment {
      units: HashMap::new(),
      next_unit_key: 1,
      items: HashMap::new(),
      next_item_key: 1,
      time_passed: 0,
    }
  }

  pub fn add_unit(&mut self, unit: Unit) {
    self.units.insert(self.next_unit_key, unit);
    self.next_unit_key += 1;
  }

  pub fn add_item(&mut self, item: Item) {
    self.items.insert(self.next_item_key, item);
    self.next_item_key += 1;
  }

  pub fn pass_time(&mut self) {
    self.time_passed += 1;
  }

  pub fn initial_spawns(&mut self, player: &RotfPlayer, unit_loader: &UnitLoader, item_loader: &ItemLoader) {
    // spawn units
    let num_units = self.num_units(player.tier());
    for _ in 0..num_units {
      let (id, level) = unit_loader.spawn();
      if id < 1 {
        continue;
      }
      let mut new_unit = Unit::new(id, level);
      new_unit.randomize_position();
      self.add_unit(new_unit);
    }
    // spawn items
    let num_items = self.num_items(player.tier());
    for _ in 0..num_items {
      let (id, level) = item_loader.spawn();
      if id < 1 {
        continue;
      }
      let mut new_item = Item::new(id, level);
      new_item.randomize_position();
      self.add_item(new_item);
    }
  }

  pub fn update(&mut self, player: &RotfPlayer, unit_loader: &UnitLoader,
    item_loader: &ItemLoader) -> Option<u64> {
    // return unit that attacks player (if any)
    let mut attacking_unit: Option<u64> = None;
    // allow units to move
    for (i, unit) in self.units.iter_mut() {
      if unit.possible_move(self.time_passed.into()) {
        attacking_unit = Some(i.clone());
      }
    }
    // despawn units
    self.units.retain(|_, u| !u.despawn());
    // respawn units
    let num_units = self.num_units(player.tier());
    if num_units > self.units.len() {
      let unit_spawns = num_units - self.units.len();
      for _ in 0..unit_spawns {
        let (id, level) = unit_loader.spawn();
        if id < 1 {
          continue;
        }
        self.add_unit(Unit::new(id, level));
      }
    }
    // allow items to move
    for (_, item) in self.items.iter_mut() {
      item.possible_move(self.time_passed.into());
    }
    // despawn items
    self.items.retain(|_, i| !i.despawn());
    // respawn items
    let num_items = self.num_items(player.tier());
    if num_items > self.items.len() {
      let item_spawns = num_items - self.items.len();
      for _ in 0..item_spawns {
        let (id, level) = item_loader.spawn();
        if id < 1 {
          continue;
        }
        self.add_item(Item::new(id, level));
      }
    }
    // reset time
    self.time_passed = 0;
    // return attacking unit
    return attacking_unit;
  }

  fn num_units(&self, tier: u8) -> usize {
    match tier {
      1 => 10,
      _ => 0,
    }
  }

  fn num_items(&self, tier: u8) -> usize {
    match tier {
      1 => 3,
      _ => 0,
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    // environment
    contents += &format!("\ntime_passed: {}", self.time_passed);
    // units
    contents += "\n";
    for (i, unit) in &self.units {
      contents += &format!("\nnext_unit_key: {}", i.clone());
      contents += "\n%%% BEGIN UNIT";
      contents += &unit.file_content();
      contents += "\n%%% END UNIT\n";
    }
    contents += &format!("\nnext_unit_key: {}", self.next_unit_key);
    // items
    contents += "\n";
    for (i, item) in &self.items {
      contents += &format!("\nnext_item_key: {}", i.clone());
      contents += "\n%%% BEGIN ITEM";
      contents += &item.file_content();
      contents += "\n%%% END ITEM\n";
    }
    contents += &format!("\nnext_unit_key: {}", self.next_item_key);
    return contents;
  }

  pub fn load(&mut self, save_name: String) -> Result<(), Error> {
    let mut in_unit = false;
    let mut in_item = false;
    let mut curr_unit = Unit::new(0, 0);
    let mut curr_item = Item::new(0, 0);
    for oline in filesystem::open_file(format!("data/saves/{}/environment.rotf", save_name))?.lines() {
      let line = oline?;
      match line.trim() {
        "%%% BEGIN UNIT" => {
          in_unit = true;
        }
        "%%% END UNIT" => {
          in_unit = false;
          self.add_unit(curr_unit);
          curr_unit = Unit::new(0, 0);
        }
        "%%% BEGIN ITEM" => {
          in_item = true;
        }
        "%%% END ITEM" => {
          in_item = false;
          self.add_item(curr_item);
          curr_item = Item::new(0, 0);
        }
        _ => {},
      }
      if !line.clone().contains(":") {
        continue;
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
        "time_passed" => self.time_passed = value.parse::<u8>().unwrap_or(0),
        _ => {},
      }
    }
    Ok(())
  }
}
