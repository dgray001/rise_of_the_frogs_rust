use crate::commands::Command;
use crate::game::RotfGame;

use std::collections::HashMap;

#[derive(Clone)]
pub enum ContextState {
  HOME,
}

pub struct RotfContext {
  pub context_state: ContextState,
  pub all_commands: HashMap<String, Command>,
  pub commands: HashMap<String, Command>,
  pub last_cmd: String,
  pub last_params: String,

  pub curr_game: Option<RotfGame>,
}