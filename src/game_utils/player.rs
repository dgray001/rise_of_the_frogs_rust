use std::{io::{Error, BufRead}, str::FromStr};

use crate::{commands::Command, filesystem};

use super::environment::{Positionable, Position};
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
    return vec![Command::VIEW, Command::FIGHT, Command::PICKUP, Command::INVENTORY];
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
    return contents;
  }

  pub fn load(&mut self, save_name: String) -> Result<(), Error> {
    for oline in filesystem::open_file(format!("data/saves/{}/player.rotf", save_name))?.lines() {
      let line = oline?;
      if !line.clone().contains(":") {
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
      _ => {},
    }
  }
}
