use std::{io::{Error, BufRead}, str::FromStr};

use crate::{commands::Command, filesystem};

use super::environment::Position;
use super::traits::Positionable;
use super::item::Item;
use super::inventory::Inventory;

pub struct RotfPlayer {
  pub level: u8,
  pub view_distance: Position,
  pub inventory: Inventory,
}

impl RotfPlayer {
  pub fn new() -> RotfPlayer {
    return RotfPlayer {
      level: 0,
      view_distance: Position::NEAR,
      inventory: Inventory::new(),
    }
  }

  pub fn environment_commands(&self) -> Vec<Command> {
    return vec![Command::VIEW, Command::WAIT, Command::FIGHT, Command::PICKUP,
      Command::INVENTORY, Command::DROP];
  }

  pub fn can_view(&self, thing: &dyn Positionable) -> bool {
    return self.view_distance.distance() >= thing.position().distance();
  }

  pub fn tier(&self) -> u8 {
    return 1 + self.level / 10;
  }

  pub fn me(&self, name: String) -> String {
    let mut str = String::new();
    str += "Player Info";
    str += &format!("\n   Name: {}", name);
    str += &format!("\n  Level: {}", self.level);
    str += &format!("\n   Tier: {}", self.tier());
    return str;
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\nlevel: {}", self.level.clone());
    contents += &format!("\nview_distance: {}", self.view_distance);
    // inventory
    contents += "\n";
    contents += &format!("\ncapacity: {}", self.inventory.capacity.clone());
    for (key, item) in &self.inventory.items {
      contents += &format!("\nnext_item_key: {}", key.clone());
      contents += "\n%%% BEGIN ITEM";
      contents += &item.file_content();
      contents += "\n%%% END ITEM\n";
    }
    contents += &format!("\nnext_item_key: {}", self.inventory.next_item_key.clone());
    return contents;
  }

  pub fn load(&mut self, save_name: String) -> Result<(), Error> {
    let mut in_item = false;
    let mut curr_item = Item::new(0, 0);
    for oline in filesystem::open_file(format!("data/saves/{}/player.rotf", save_name))?.lines() {
      let line = oline?;
      match line.trim() {
        "%%% BEGIN ITEM" => {
          in_item = true;
        }
        "%%% END ITEM" => {
          in_item = false;
          self.inventory.add(curr_item);
          curr_item = Item::new(0, 0);
        }
        _ => {},
      }
      if !line.clone().contains(":") {
        continue;
      }
      if in_item {
        curr_item.read_line(line);
        continue;
      }
      let (key, mut value) = line.split_once(":").unwrap();
      value = value.trim();
      self.read_line(key.trim(), value);
    }
    Ok(())
  }

  pub fn read_line(&mut self, key: &str, value: &str) {
    match key {
      "level" => self.level = value.parse::<u8>().unwrap_or(0),
      "view_distance" => self.view_distance = Position::from_str(value).unwrap_or(Position::FAR),
      "capacity" => self.inventory.capacity = value.parse::<usize>().unwrap_or(0),
      "next_item_key" => self.inventory.next_item_key = value.parse::<u64>().unwrap_or(1),
      _ => {},
    }
  }
}
