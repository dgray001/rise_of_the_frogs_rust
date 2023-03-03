mod system_commands;

use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::context::RotfContext;

pub fn parse_command(name: &str, context: &mut RotfContext) {
  let guaranteed_split = name.to_owned() + " ";
  let name_split = guaranteed_split.split_once(" ").unwrap();
  let last_cmd = name_split.0.trim().to_lowercase();
  context.last_cmd = last_cmd.clone();
  context.last_params = name_split.1.trim().to_lowercase();
  let commands = context.commands.clone();
  match commands.get(&last_cmd) {
    Some(cmd) => cmd.call(context),
    None => println!("Invalid command."),
  }
  context.commands = get_current_commands();
}

#[derive(Clone, Debug, EnumIter)]
pub enum Command {
  LS,
  HELP,
  EXIT,
}

impl Command {
  const fn name(&self) -> &'static str {
    match *self {
      Command::LS => "ls",
      Command::HELP => "help",
      Command::EXIT => "exit",
    }
  }
  const fn description(&self) -> &'static str {
    match *self {
      Command::LS => "List available commands",
      Command::HELP => "Display helptext about the specified",
      Command::EXIT => "Exit the program",
    }
  }
  const fn helptext(&self) -> &'static str {
    match *self {
      Command::LS => "ls",
      Command::HELP => "help",
      Command::EXIT => "exit",
    }
  }
  const fn aliases(&self) -> Vec<&'static str> {
    match *self {
      Command::LS => vec![],
      Command::HELP => vec![],
      Command::EXIT => vec![],
    }
  }
  fn call(&self, context: &mut RotfContext) {
    match *self {
      Command::LS => system_commands::ls(context),
      Command::HELP => system_commands::help(context),
      Command::EXIT => system_commands::exit(context),
    }
  }
}

pub fn get_current_commands() -> HashMap<String, Command> {
  let mut cmds = HashMap::new();
  for cmd in Command::iter() {
    cmds.insert(cmd.name().to_string(), cmd.clone());
    for alias in cmd.aliases() {
      cmds.insert(alias.to_string(), cmd.clone());
    }
  }
  /*for cmd in get_system_commands() {
    cmds.insert(String::from(cmd.name), cmd.copy());
    for alias in cmd.aliases {
      cmds.insert(String::from("test"), &cmd);
    }
  }*/
  return cmds;
}