mod system_commands;
mod context_state_commands;

use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{context::{RotfContext, ContextState}, credits};

pub fn parse_command(name: &str, context: &mut RotfContext) {
  let guaranteed_split = name.to_owned() + " ";
  let name_split = guaranteed_split.split_once(" ").unwrap();
  let last_cmd = name_split.0.trim().to_lowercase();
  context.last_cmd = last_cmd.clone();
  context.last_params = name_split.1.trim().to_lowercase();
  let commands = context.commands.clone();
  match commands.get(&last_cmd) {
    Some(cmd) => cmd.call(context),
    None => eprintln!("Invalid command."),
  }
  context.commands = get_current_commands(context);
}

#[derive(Clone, Debug, EnumIter)]
pub enum Command {
  LS,
  HELP,
  EXIT,
  CREDITS,
  LAUNCH,
}

impl Command {
  fn system_commands() -> Vec<Command> {
    return vec![Command::LS, Command::HELP, Command::EXIT, Command::CREDITS];
  }
  fn context_state_commands(state: ContextState) -> Vec<Command> {
    match state {
      ContextState::HOME => vec![Command::LAUNCH],
    }
  }

  const fn name(&self) -> &'static str {
    match *self {
      Command::LS => "ls",
      Command::HELP => "help",
      Command::EXIT => "exit",
      Command::CREDITS => "credits",
      Command::LAUNCH => "launch",
    }
  }
  const fn description(&self) -> &'static str {
    match *self {
      Command::LS => "List available commands",
      Command::HELP => "Display helptext about the specified",
      Command::EXIT => "Exit the program",
      Command::CREDITS => "Display the credits",
      Command::LAUNCH => "Launches a new or saved game",
    }
  }
  fn helptext(&self) {
    println!("Command: {}", self.name());
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
      Command::LAUNCH => {
        println!("Usage: 'launch {{arg}}'");
        println!("Launch a new game with 'launch new'");
        println!("Launch a saved game with 'launch {{saved_game_name}}");
        println!("View the list of saved games with 'launch ls'");
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
      Command::LAUNCH => context_state_commands::launch(&context.last_params),
    }
  }
}

pub fn get_all_commands() -> HashMap<String, Command> {
  let mut cmds = HashMap::new();
  for cmd in Command::iter() {
    cmds.insert(cmd.name().to_string(), cmd.clone());
  }
  return cmds;
}

pub fn get_current_commands(context: &RotfContext) -> HashMap<String, Command> {
  let mut cmds = HashMap::new();
  let mut all_cmds: Vec<Command> = Vec::new();
  all_cmds.append(&mut Command::system_commands());
  all_cmds.append(&mut Command::context_state_commands(context.context_state.clone()));
  for cmd in all_cmds {
    cmds.insert(cmd.name().to_string(), cmd.clone());
    for alias in cmd.aliases() {
      cmds.insert(alias.to_string(), cmd.clone());
    }
  }
  return cmds;
}