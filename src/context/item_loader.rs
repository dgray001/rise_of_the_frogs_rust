use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, Error};

use rand::seq::SliceRandom;

use crate::game::player::RotfPlayer;
use crate::numeric::{IntegerRange, random_int};
use crate::filesystem;

use super::constants;


// Service struct that parses item data and delivers it to context
pub struct ItemLoader {
  item_data: HashMap<u64, ItemData>, // all items
  error_item_data: ItemData,
  current_items: Vec<u64>, // spawnable items (in level range)
  current_level: u8,
}

impl ItemLoader {
  pub fn new() -> ItemLoader {
    return ItemLoader {
      item_data: HashMap::new(),
      error_item_data: ItemData::new(),
      current_items: Vec::new(),
      current_level: 0,
    }
  }

  pub fn load_data(&mut self) -> Result<(), Error> {
    for oline in filesystem::open_file(format!("data/items/data.csv"))?.lines() {
      let data: Vec<String> = oline?.trim().split(",").map(|s| s.to_string()).collect();
      if data.len() < 4 {
        continue;
      }
      let mut item = ItemData::new();
      // id
      match data.get(1).unwrap().parse::<u64>() {
        Ok(id) => {
          if id < 1 {
            continue;
          }
          item.id = id;
        },
        Err(_) => {
          continue;
        }
      }
      // name
      item.name = data.get(2).unwrap().trim().to_owned();
      if item.name.is_empty() {
        continue;
      }
      // level range
      item.level_range = IntegerRange::from_str(data.get(3).unwrap());
      // add to item data
      match self.item_data.insert(item.id, item) {
        Some(previous_item) => {
          return Err(Error::new(std::io::ErrorKind::Other, format!("Duplicate item id {}", previous_item.id)));
        },
        None => {},
      }
    }
    Ok(())
  }

  pub fn update_current_items(&mut self, player: &RotfPlayer) {
    self.current_level = player.level;
    self.current_items.clear();
    for (id, item) in &self.item_data {
      if item.level_range.contains(self.current_level.into()) {
        self.current_items.push(*id);
      }
    }
  }

  pub fn spawn(&self) -> (u64, u8) {
    match self.current_items.choose(&mut rand::thread_rng()) {
      Some(id) => {
        let item = self.item_data.get(id).unwrap_or(&self.error_item_data);
        let mut min_player_level = self.current_level;
        if self.current_level > constants::ITEM_SPAWN_RANGE_MIN {
          min_player_level -= constants::ITEM_SPAWN_RANGE_MIN;
        }
        else {
          min_player_level = 0;
        }
        let min_level = min(min_player_level,
          item.level_range.min().try_into().unwrap_or(0));
        let mut max_player_level = self.current_level;
        if self.current_level > constants::ITEM_SPAWN_RANGE_MAX {
          max_player_level -= constants::ITEM_SPAWN_RANGE_MAX;
        }
        else {
          max_player_level = 0;
        }
        let max_level = min(max_player_level,
          item.level_range.max().try_into().unwrap_or(0));
        return (*id, random_int(min_level, max_level));
      },
      None => (0, 0),
    }
  }

  pub fn get_data(&self, id: u64) -> &ItemData {
    return self.item_data.get(&id).unwrap_or(&self.error_item_data);
  }
}


// Struct to hold a single item's data
pub struct ItemData {
  pub id: u64,
  pub name: String,
  pub level_range: IntegerRange,
}

impl ItemData {
  fn new() -> ItemData {
    return ItemData {
      id: 0,
      name: "".to_owned(),
      level_range: IntegerRange::new(),
    }
  }
}