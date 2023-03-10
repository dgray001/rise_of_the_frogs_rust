use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{filesystem, commands::Command};

use core::fmt;
use std::{io::{Error, BufRead}, str::FromStr};

#[derive(Debug, EnumIter)]
pub enum GameState {
  CUTSCENE,
}

impl fmt::Display for GameState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for GameState {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for state in GameState::iter() {
      if state.to_string() == s {
        return Ok(state);
      }
    }
    Err(())
  }
}

pub struct RotfGame {
  pub name: String,
  pub state: GameState,
}

impl RotfGame {
  pub fn new(name: String) -> RotfGame {
    return RotfGame {
      name,
      state: GameState::CUTSCENE,
    }
  }

  pub fn load(name: String) -> Result<RotfGame, Error> {
    let mut game = RotfGame::new(name.clone());
    for oline in filesystem::open_file(format!("data/saves/{}/metadata.rotf", name))?.lines() {
      let line = oline?;
      if !line.clone().contains(":") {
        continue;
      }
      let (key, mut value) = line.split_once(":").unwrap();
      value = value.trim();
      match key.trim() {
        "name" => game.name = value.to_string(),
        "state" => game.state = GameState::from_str(value).unwrap_or(GameState::CUTSCENE),
        _ => {},
      }
    }
    Ok(game)
  }

  pub fn save(&self) -> Result<(), Error> {
    filesystem::create_folder(format!("data/saves/{}", self.name))?;
    filesystem::create_file(format!("data/saves/{}/metadata.rotf", self.name), self.metadata_content())?;
    Ok(())
  }
  
  fn metadata_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\nname: {}", self.name.clone());
    contents += &format!("\nstate: {}", self.state);
    return contents;
  }

  pub fn commands(&self) -> Vec<Command> {
    return vec![];
  }
}