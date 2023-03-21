use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, Error};

use rand::seq::SliceRandom;

use crate::game::player::RotfPlayer;
use crate::numeric::{IntegerRange, random_int};
use crate::filesystem;

use super::constants;


// Service struct that parses non-player unit data and delivers it to context
pub struct UnitLoader {
  unit_data: HashMap<u64, UnitData>, // all units
  error_unit_data: UnitData,
  current_units: Vec<u64>, // spawnable units (in level range)
  current_level: u8,
}

impl UnitLoader {
  pub fn new() -> UnitLoader {
    return UnitLoader {
      unit_data: HashMap::new(),
      error_unit_data: UnitData::new(),
      current_units: Vec::new(),
      current_level: 0,
    }
  }

  pub fn load_data(&mut self) -> Result<(), Error> {
    for oline in filesystem::open_file(format!("data/units/data.csv"))?.lines() {
      let data: Vec<String> = oline?.trim().split(",").map(|s| s.to_string()).collect();
      if data.len() < 4 {
        continue;
      }
      let mut unit = UnitData::new();
      // id
      match data.get(1).unwrap().parse::<u64>() {
        Ok(id) => {
          if id < 1 {
            continue;
          }
          unit.id = id;
        },
        Err(_) => {
          continue;
        }
      }
      // name
      unit.name = data.get(2).unwrap().trim().to_owned();
      if unit.name.is_empty() {
        continue;
      }
      // level range
      unit.level_range = IntegerRange::from_str(data.get(3).unwrap());
      // add to unit data
      match self.unit_data.insert(unit.id, unit) {
        Some(previous_unit) => {
          return Err(Error::new(std::io::ErrorKind::Other, format!("Duplicate unit id {}", previous_unit.id)));
        },
        None => {},
      }
    }
    Ok(())
  }

  pub fn update_current_units(&mut self, player: &RotfPlayer) {
    self.current_level = player.level;
    self.current_units.clear();
    for (id, unit) in &self.unit_data {
      if unit.level_range.contains(self.current_level.into()) {
        self.current_units.push(*id);
      }
    }
  }

  pub fn spawn(&self) -> (u64, u8) {
    match self.current_units.choose(&mut rand::thread_rng()) {
      Some(id) => {
        let unit = self.unit_data.get(id).unwrap_or(&self.error_unit_data);
        let mut min_player_level = self.current_level;
        if self.current_level > 5 {
          min_player_level -= constants::UNIT_SPAWN_RANGE;
        }
        else {
          min_player_level = 0;
        }
        let min_level = min(min_player_level,
          unit.level_range.min().try_into().unwrap_or(0));
        let max_level = min(self.current_level + constants::UNIT_SPAWN_RANGE,
          unit.level_range.max().try_into().unwrap_or(0));
        return (*id, random_int(min_level, max_level));
      },
      None => (0, 0),
    }
  }

  pub fn get_data(&self, id: u64) -> &UnitData {
    return self.unit_data.get(&id).unwrap_or(&self.error_unit_data);
  }
}


// Struct to hold a single unit's data
pub struct UnitData {
  pub id: u64,
  pub name: String,
  pub level_range: IntegerRange,
}

impl UnitData {
  fn new() -> UnitData {
    return UnitData {
      id: 0,
      name: "".to_owned(),
      level_range: IntegerRange::new(),
    }
  }
}