use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::context::item_loader::ItemLoader;
use crate::context::unit_loader::UnitLoader;
use crate::filesystem;
use crate::commands::Command;
use crate::cutscene;

use std::fmt;
use std::io::{Error, BufRead};
use std::str::FromStr;

use self::combat::RotfCombat;

pub mod player;
pub mod environment;
pub mod traits;
mod unit;
mod item;
mod inventory;
mod ability;
mod combat;


// GameState determines available commands
#[derive(Debug, EnumIter, PartialEq)]
pub enum GameState {
  CUTSCENE,
  ENVIRONMENT,
  COMBAT,
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


// Struct containing information necessary to identify a unit
pub struct UnitIdentifier {
  is_player: bool,
  unit_index: u64,
}

impl UnitIdentifier {
  pub fn new() -> UnitIdentifier {
    return UnitIdentifier {
      is_player: false,
      unit_index: 0,
    }
  }
}


// RotfGame is a struct with all game information, including the environment and player
pub struct RotfGame {
  pub name: String,
  pub state: GameState,
  pub difficulty: RotfDifficulty,
  pub last_cutscene: cutscene::RotfCutscene,

  pub player: player::RotfPlayer,
  pub environment: environment::RotfEnvironment,
  pub combat: Option<combat::RotfCombat>,
}

impl RotfGame {
  pub fn new(name: String, difficulty: RotfDifficulty) -> RotfGame {
    return RotfGame {
      name,
      state: GameState::CUTSCENE,
      difficulty,
      last_cutscene: cutscene::RotfCutscene::LAUNCH_GAME,
      player: player::RotfPlayer::new(),
      environment: environment::RotfEnvironment::new(),
      combat: None,
    }
  }

  pub fn enter_combat(&mut self, unit_index: u64, player_start: bool) {
    let mut combat = RotfCombat::new();
    combat.add_team("player", vec![UnitIdentifier {
      is_player: true,
      unit_index: 0,
    }]);
    combat.add_team("enemy", vec![UnitIdentifier {
      is_player: false,
      unit_index,
    }]);
    if player_start {
      combat.turn = 0;
    }
    else {
      combat.turn = 1;
    }
    self.combat = Some(combat);
    self.state = GameState::COMBAT;
  }

  pub fn initial_spawns(&mut self, unit_loader: &UnitLoader, item_loader: &ItemLoader) {
    self.environment.initial_spawns(&self.player, unit_loader, item_loader);
  }

  pub fn commands(&self) -> Vec<Command> {
    match self.state {
      GameState::CUTSCENE => vec![],
      GameState::ENVIRONMENT => self.player.environment_commands(),
      GameState::COMBAT => self.player.combat_commands(),
    }
  }

  pub fn update(&mut self, unit_loader: &UnitLoader, item_loader: &ItemLoader) {
    match self.state {
      GameState::ENVIRONMENT => {
        match self.environment.update(&self.player, unit_loader, item_loader) {
          Some(i) => {
            self.enter_combat(i, false);
          },
          None => {},
        }
      },
      _ => {},
    }
  }

  pub fn load(name: String) -> Result<RotfGame, Error> {
    let save_name = str::replace(name.as_str(), " ", "_");
    let mut game = RotfGame::new(save_name.clone(), RotfDifficulty::default());
    // load metadata
    for oline in filesystem::open_file(format!("data/saves/{}/metadata.rotf", save_name))?.lines() {
      let line = oline?;
      if !line.clone().contains(":") {
        continue;
      }
      let (key, mut value) = line.split_once(":").unwrap();
      value = value.trim();
      match key.trim() {
        "name" => game.name = value.to_owned(),
        "state" => game.state = GameState::from_str(value).unwrap_or(GameState::CUTSCENE),
        "difficulty" => game.difficulty = RotfDifficulty::from_str(value).unwrap_or(RotfDifficulty::default()),
        _ => {},
      }
    }
    // load player
    game.player.load(save_name.clone())?;
    // load environment
    game.environment.load(save_name.clone())?;
    Ok(game)
  }

  pub fn save(&self) -> Result<(), Error> {
    let save_name = str::replace(self.name.as_str(), " ", "_");
    filesystem::create_folder(format!("data/saves/{}", save_name))?;
    filesystem::create_file(format!("data/saves/{}/metadata.rotf", save_name), self.metadata_content())?;
    filesystem::create_file(format!("data/saves/{}/player.rotf", save_name), self.player.file_content())?;
    filesystem::create_file(format!("data/saves/{}/environment.rotf", save_name), self.environment.file_content())?;
    Ok(())
  }
  
  fn metadata_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\nname: {}", self.name.clone());
    contents += &format!("\nstate: {}", self.state);
    contents += &format!("\ndifficulty: {}", self.difficulty);
    return contents;
  }
}
