#[path = "command_utils/commands.rs"] mod commands;
#[path = "game_utils/game.rs"] mod game;
#[path = "utils/filesystem.rs"] mod filesystem;

mod credits;
mod context;

use context::RotfContext;

use std::{io::{self, Write}, collections::HashMap};

fn main() {
  credits::welcome();

  let mut context: RotfContext = RotfContext {
    context_state: context::ContextState::HOME,
    all_commands: commands::get_all_commands(),
    commands: HashMap::new(),
    last_cmd: "".to_string(),
    last_params: "".to_string(),

    curr_game: None,
  };
  context.commands = commands::get_current_commands(&context);

  loop {
    println!("-------------------");
    println!("");
    print!(" > ");
    io::stdout().flush().unwrap();
    let mut cmd = String::new();
    match io::stdin().read_line(&mut cmd) {
      Ok(_n) => {
        println!();
        commands::parse_command(&cmd, &mut context);
      },
      Err(e) => {
        println!("Error reading command: {}", e);
        println!();
      },
    }
  }
}