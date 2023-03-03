use crate::commands::Command;

use std::collections::HashMap;

pub struct RotfContext {
  pub commands: HashMap<String, Command>,
  pub last_cmd: String,
  pub last_params: String,
}