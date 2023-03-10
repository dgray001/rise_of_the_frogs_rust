use crate::filesystem;

use std::io::Error;

pub enum GameState {
  CUTSCENE,
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

  pub fn save(&self) -> Result<(), Error> {
    filesystem::create_folder(format!("data/saves/{}", self.name))?;
    filesystem::create_file(format!("data/saves/{}/metadata.rotf", self.name), self.metadata_content())?;
    Ok(())
  }
  
  fn metadata_content(&self) -> String {
    return self.name.clone();
  }
}