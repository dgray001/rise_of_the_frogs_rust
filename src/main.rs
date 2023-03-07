#[path = "command_utils/commands.rs"] mod commands;
#[path = "game_utils/game.rs"] mod game;
#[path = "utils/filesystem.rs"] mod filesystem;

mod credits;
mod context;
mod main_test;

use context::RotfContext;

use std::io::{self, BufRead, Write};

fn main() {
  credits::welcome();

  main_loop(io::stdin().lock(), io::stdout(), io::stderr());
}

fn main_loop<R, W, E>(mut input: R, mut output: W, mut error: E) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let mut context = RotfContext::default_context(input, output, error);

  loop {
    context.println("-------------------");
    context.println("");
    context.print(" > ");
    match context.read_line() {
      Ok(cmd) => {
        println!();
        commands::parse_command(&cmd, &mut context);
      },
      Err(e) => {
        context.print_error("reading command", &e);
        context.println("");
      },
    }
  }
}