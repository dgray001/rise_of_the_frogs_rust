#[path = "command_utils/commands.rs"] mod commands;
#[path = "game_utils/game.rs"] mod game;

mod credits;
mod context;

use commands::get_current_commands;
use context::RotfContext;

use std::io;

fn main() {
  credits::welcome();

  let mut context: RotfContext = RotfContext {
    context_state: context::ContextState::HOME,
    commands: get_current_commands(),
    last_cmd: "".to_string(),
    last_params: "".to_string(),

    curr_game: None,
  };

  loop {
    println!("-------------------");
    println!("");
    let mut cmd = String::new();
    io::stdin()
      .read_line(&mut cmd)
      .expect("Failed to read line");
    println!();
    commands::parse_command(&cmd, &mut context);
  }
}