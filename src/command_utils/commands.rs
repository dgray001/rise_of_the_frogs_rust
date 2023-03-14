mod system_commands;
mod context_state_commands;
mod environment_commands;
  
use std::{collections::HashMap, io::{Write, BufRead}};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{context::{RotfContext, ContextState}, credits, cutscene::RotfCutscene};

pub fn parse_command<R, W, E>(cmd: &str, context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  // Prepare command
  if cmd.trim().is_empty() {
    return;
  }
  let guaranteed_split = cmd.to_owned() + " ";
  let cmd_split = guaranteed_split.split_once(" ").unwrap();
  let last_cmd = cmd_split.0.trim().to_lowercase();
  context.last_cmd = last_cmd.clone();
  context.last_params = cmd_split.1.trim().to_lowercase();
  let commands = context.commands.clone();
  // Run command
  match commands.get(&last_cmd) {
    Some(cmd) => cmd.call(context),
    None => context.eprintln("Invalid command"),
  }
  // Play cutscene if relevant
  RotfCutscene::resolve_context(context);
  // Save game
  match &context.curr_game {
    Some(game) => match game.save() {
      Ok(()) => {},
      Err(e) => context.print_error("saving game", &e),
    },
    None => {},
  }
  // Get current commands
  context.commands = get_current_commands(context);
}

#[derive(Clone, Debug, EnumIter)]
pub enum Command {
  // System Commands
  LS,
  HELP,
  EXIT,
  CREDITS,
  REPLAY,
  OPTIONS,
  // ContextState::HOME Commands
  LAUNCH,
  DELETE,
  // ContextState::INGAME Commands
  ME,
  SAVE,
  // GameState::ENVIRONMENT Commands
  VIEW,
  TURN,
  MOVE,
  FIGHT,
  PICKUP,
  INVENTORY,
  // GameState::COMBAT Commands
  FLEE,
}

impl Command {
  fn system_commands() -> Vec<Command> {
    return vec![Command::LS, Command::HELP, Command::EXIT, Command::CREDITS, Command::REPLAY, Command::OPTIONS];
  }
  fn context_state_commands<R, W, E>(context: &mut RotfContext<R, W, E>) -> Vec<Command> where
    R: BufRead,
    W: Write,
    E: Write,
  {
    match context.context_state {
      ContextState::HOME => vec![Command::LAUNCH, Command::DELETE],
      ContextState::INGAME => {
        let mut context_cmds = vec![Command::ME, Command::SAVE];
        let mut game_cmds = match &context.curr_game {
          Some(game) => game.commands(),
          None => vec![],
        };
        context_cmds.append(&mut game_cmds);
        context_cmds
      },
    }
  }

  const fn name(&self) -> &'static str {
    match *self {
      // System Commands
      Command::LS => "ls",
      Command::HELP => "help",
      Command::EXIT => "exit",
      Command::CREDITS => "credits",
      Command::REPLAY => "replay",
      Command::OPTIONS => "options",
      // ContextState::HOME Commands
      Command::LAUNCH => "launch",
      Command::DELETE => "delete",
      // ContextState::INGAME Commands
      Command::ME => "me",
      Command::SAVE => "save",
      // GameState::ENVIRONMENT Commands
      Command::VIEW => "view",
      Command::FIGHT => "fight",
      Command::PICKUP => "pickup",
      Command::INVENTORY => "inventory",
      // GameState::COMBAT Commands
      _ => "",
    }
  }
  const fn description(&self) -> &'static str {
    match *self {
      // System Commands
      Command::LS => "List available commands",
      Command::HELP => "Display helptext about the specified",
      Command::EXIT => "Exit the program",
      Command::CREDITS => "Display the credits",
      Command::REPLAY => "Replay last cutscene",
      Command::OPTIONS => "Opens the options menu",
      // ContextState::HOME Commands
      Command::LAUNCH => "Launches a new or saved game",
      Command::DELETE => "Delete the specified saved game",
      // ContextState::INGAME Commands
      Command::ME => "Display info about the current player",
      Command::SAVE => "Save your progress and return to the main menu",
      // GameState::ENVIRONMENT Commands
      Command::VIEW => "View your surroundings",
      Command::FIGHT => "Fight the specified unit in your view",
      Command::PICKUP => "Pickup the specified item in your view",
      Command::INVENTORY => "View your inventory",
      // GameState::COMBAT Commands
      _ => "Not implemented",
    }
  }
  fn helptext<R, W, E>(&self, context: &mut RotfContext<R, W, E>) where
    R: BufRead,
    W: Write,
    E: Write,
  {
    context.print_data("Command: {}", self.name());
    if !self.aliases().is_empty() {
      context.print_data("Aliases: {}", self.aliases().join(", "));
    }
    match *self {
      // System Commands
      Command::LS => {
        context.println("Lists all available commands and a short description of how they work");
      },
      Command::HELP => {
        context.println("Usage: 'help {{arg}}'");
        context.println("Prints helptext related to the specified arg");
        context.println("If no arg is specified will print general helptext");
      },
      Command::EXIT => {
        context.println("Exit the program");
      },
      Command::CREDITS => {
        context.println("Display credits for the game");
      },
      Command::REPLAY => {
        context.println("Usage: 'replay {{arg}}'");
        context.println("Replay last cutscene with 'replay cutscene'");
      },
      Command::OPTIONS => {
        context.println("Opens the options menu, where options can be saved");
        context.println("Options will be persistent across saves");
      },
      // ContextState::HOME Commands
      Command::LAUNCH => {
        context.println("Usage: 'launch {{arg}}'");
        context.println("Launch a new game with 'launch new'");
        context.println("Launch a saved game with 'launch {{saved_game_name}}");
        context.println("View the list of saved games with 'launch ls'");
      },
      Command::DELETE => {
        context.println("Usage: 'delete {{arg}}'");
        context.println("Delete an existing saved game permanently");
      },
      // ContextState::INGAME Commands
      Command::ME => {
        context.println("Displays info about the player");
      },
      Command::SAVE => {
        context.println("Saves your progress and returns to the main menu");
        context.println("Since the game saves itself as you play, this command is more so you can switch save games");
      },
      // GameState::ENVIRONMENT Commands
      Command::VIEW => {
        context.println("View your current surroundings");
        context.println("You will see things based on how far you can view");
      },
      
      Command::FIGHT => {
        context.println("Usage: 'fight {{arg}}'");
        context.println("Arg is the index of the viewable unit to fight");
        context.println("To see the viewable index of units you can fight, use 'view'");
      },
      Command::PICKUP => {
        context.println("Usage: 'pickup {{arg}}'");
        context.println("Arg is the index of the viewable item to pickup");
        context.println("To see the viewable index of items you can pickup, use 'view'");
      },
      Command::INVENTORY => {
        context.println("View the contents of your inventory");
      },
      // GameState::COMBAT Commands
      _ => {
        context.println("Not implemented.");
      },
    }
  }
  fn aliases(&self) -> Vec<&'static str> {
    match *self {
      Command::LS => vec!["list"],
      Command::HELP => vec!["?"],
      Command::EXIT => vec!["quit"],
      Command::VIEW => vec!["vw"],
      Command::FIGHT => vec!["fi"],
      Command::PICKUP => vec!["pi"],
      Command::INVENTORY => vec!["inv"],
      _ => vec![],
    }
  }
  fn call<R, W, E>(&self, context: &mut RotfContext<R, W, E>) where
    R: BufRead,
    W: Write,
    E: Write,
  {
    match *self {
      // System Commands
      Command::LS => system_commands::ls(context),
      Command::HELP => system_commands::help(context),
      Command::EXIT => context.exit = true,
      Command::CREDITS => credits::credits(context),
      Command::REPLAY => system_commands::replay(context),
      Command::OPTIONS => system_commands::options(context),
      // ContextState::HOME Commands
      Command::LAUNCH => context_state_commands::launch(context),
      Command::DELETE => context_state_commands::delete(context),
      // ContextState::INGAME Commands
      Command::ME => context_state_commands::me(context),
      Command::SAVE => context_state_commands::save(context),
      // GameState::ENVIRONMENT Commands
      Command::VIEW | Command::FIGHT | Command::PICKUP | Command::INVENTORY => {
        environment_commands::command(context, self.name());
      },
      // GameState::COMBAT Commands
      _ => {},
    }
  }
}

pub fn get_all_commands() -> HashMap<String, Command> {
  let mut cmds = HashMap::new();
  for cmd in Command::iter() {
    cmds.insert(cmd.name().to_owned(), cmd.clone());
  }
  return cmds;
}

pub fn get_current_commands<R, W, E>(context: &mut RotfContext<R, W, E>) -> HashMap<String, Command> where
  R: BufRead,
  W: Write,
  E: Write,
{
  let mut cmds = HashMap::new();
  let mut all_cmds: Vec<Command> = Vec::new();
  all_cmds.append(&mut Command::system_commands());
  all_cmds.append(&mut Command::context_state_commands(context));
  for cmd in all_cmds {
    cmds.insert(cmd.name().to_owned(), cmd.clone());
    for alias in cmd.aliases() {
      cmds.insert(alias.to_owned(), cmd.clone());
    }
  }
  return cmds;
}
