mod system_commands;

use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{context::RotfContext, credits};

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
  CREDITS,
}

impl Command {
  const fn name(&self) -> &'static str {
    match *self {
      Command::LS => "ls",
      Command::HELP => "help",
      Command::EXIT => "exit",
      Command::CREDITS => "credits",
    }
  }
  const fn description(&self) -> &'static str {
    match *self {
      Command::LS => "List available commands",
      Command::HELP => "Display helptext about the specified",
      Command::EXIT => "Exit the program",
      Command::CREDITS => "Display the credits",
    }
  }
  fn helptext(&self) {
    if !self.aliases().is_empty() {
      println!("Aliases: {}", self.aliases().join(", "));
    }
    match *self {
      Command::LS => {
        println!("Lists all available commands and a short description of how they work");
      },
      Command::HELP => {
        println!("Usage: 'help {{arg}}'");
        println!("Prints helptext related to the specified arg");
        println!("If no arg is specified will print general helptext");
      },
      Command::EXIT => {
        println!("Exit the program");
      },
      Command::CREDITS => {
        println!("Display credits for the game");
      },
    }
  }
  fn aliases(&self) -> Vec<&'static str> {
    match *self {
      Command::LS => vec!["list"],
      Command::HELP => vec!["?"],
      Command::EXIT => vec!["quit"],
      _ => vec![],
    }
  }
  fn call(&self, context: &mut RotfContext) {
    match *self {
      Command::LS => system_commands::ls(context),
      Command::HELP => system_commands::help(context),
      Command::EXIT => system_commands::exit(),
      Command::CREDITS => credits::credits(),
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
  return cmds;
}