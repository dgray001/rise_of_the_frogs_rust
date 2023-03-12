use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{filesystem, commands::Command, cutscene};

use core::fmt;
use std::{io::{Error, BufRead}, str::FromStr};

mod player;


// GameState determines available commands
#[derive(Debug, EnumIter, PartialEq)]
pub enum GameState {
  CUTSCENE,
  ENVIRONMENT,
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


// RotfDifficulty determines a factor for the strenth of opponents
#[derive(Debug, EnumIter, PartialEq)]
pub enum RotfDifficulty {
  PEACEFUL,
  EASY,
  NORMAL,
  HARD,
}

impl RotfDifficulty {
  pub fn default() -> RotfDifficulty {
    return RotfDifficulty::NORMAL;
  }
}

impl fmt::Display for RotfDifficulty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for RotfDifficulty {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for state in RotfDifficulty::iter() {
      if state.to_string() == s {
        return Ok(state);
      }
    }
    Err(())
  }
}


// RotfGame is a struct with all game information, including the environment and player
pub struct RotfGame {
  pub name: String,
  pub state: GameState,
  pub difficulty: RotfDifficulty,
  pub last_cutscene: cutscene::RotfCutscene,

  pub player: player::RotfPlayer,
}

impl RotfGame {
  pub fn new(name: String, difficulty: RotfDifficulty) -> RotfGame {
    return RotfGame {
      name,
      state: GameState::CUTSCENE,
      difficulty,
      last_cutscene: cutscene::RotfCutscene::LAUNCH_GAME,
      player: player::RotfPlayer::new(),
    }
  }

  pub fn load(name: String) -> Result<RotfGame, Error> {
    let save_name = str::replace(name.as_str(), " ", "_");
    let mut game = RotfGame::new(save_name.clone(), RotfDifficulty::default());
    for oline in filesystem::open_file(format!("data/saves/{}/metadata.rotf", save_name))?.lines() {
      let line = oline?;
      if !line.clone().contains(":") {
        continue;
      }
      let (key, mut value) = line.split_once(":").unwrap();
      value = value.trim();
      match key.trim() {
        "name" => game.name = value.to_string(),
        "state" => game.state = GameState::from_str(value).unwrap_or(GameState::CUTSCENE),
        "difficulty" => game.difficulty = RotfDifficulty::from_str(value).unwrap_or(RotfDifficulty::default()),
        _ => {},
      }
    }
    Ok(game)
  }

  pub fn save(&self) -> Result<(), Error> {
    let save_name = str::replace(self.name.as_str(), " ", "_");
    filesystem::create_folder(format!("data/saves/{}", save_name))?;
    filesystem::create_file(format!("data/saves/{}/metadata.rotf", save_name), self.metadata_content())?;
    filesystem::create_file(format!("data/saves/{}/player.rotf", save_name), self.player.file_content())?;
    Ok(())
  }
  
  fn metadata_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\nname: {}", self.name.clone());
    contents += &format!("\nstate: {}", self.state);
    contents += &format!("\ndifficulty: {}", self.difficulty);
    return contents;
  }

  pub fn commands(&self) -> Vec<Command> {
    match self.state {
      GameState::CUTSCENE => vec![],
      GameState::ENVIRONMENT => self.player.environment_commands(),
    }
  }
}