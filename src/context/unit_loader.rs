use std::collections::HashMap;
use std::io::{BufRead, Error};

use rand::seq::SliceRandom;

use crate::game::player::RotfPlayer;
use crate::numeric::IntegerRange;
use crate::filesystem;


// Service struct that parses non-player unit data and delivers it to context
pub struct UnitLoader {
  unit_data: HashMap<u64, UnitData>, // all units
  current_units: Vec<u64>, // spawnable units (in level range)
}

impl UnitLoader {
  pub fn new() -> UnitLoader {
    return UnitLoader {
      unit_data: HashMap::new(),
      current_units: Vec::new(),
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
          unit.id = id;
        },
        Err(_) => {
          continue;
        }
      }
      // name
      unit.name = data.get(2).unwrap().to_owned();
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
    let level = player.level;
    self.current_units.clear();
    for (id, unit) in &self.unit_data {
      if unit.level_range.contains(level.into()) {
        self.current_units.push(*id);
      }
    }
  }

  pub fn spawn(&self) -> u64 {
    match self.current_units.choose(&mut rand::thread_rng()) {
      Some(id) => *id,
      None => 0,
    }
  }
}


// Struct to hold a single unit's data
struct UnitData {
  id: u64,
  name: String,
  level_range: IntegerRange,
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