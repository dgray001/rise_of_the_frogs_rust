use crate::commands::Command;

use std::collections::HashMap;

pub struct RotfContext {
  pub commands: HashMap<String, Command<'static>>,
}