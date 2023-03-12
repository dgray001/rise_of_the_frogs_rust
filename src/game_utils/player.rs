use std::io::{Error, BufRead};

use crate::{commands::Command, filesystem};

pub struct RotfPlayer {
  pub level: u8,
}

impl RotfPlayer {
  pub fn new() -> RotfPlayer {
    return RotfPlayer {
      level: 0,
    }
  }

  pub fn environment_commands(&self) -> Vec<Command> {
    return vec![];
  }

  fn tier(&self) -> u8 {
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
      _ => {},
    }
  }
}