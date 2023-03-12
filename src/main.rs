#[path = "command_utils/commands.rs"] mod commands;
#[path = "game_utils/game.rs"] mod game;
#[path = "game_utils/cutscene.rs"] mod cutscene;
#[path = "utils/filesystem.rs"] mod filesystem;
#[path = "options.rs"] mod options;

mod credits;
mod context;

use context::RotfContext;

use std::io::{self, BufRead, Write};

fn main() {
  main_loop(io::stdin().lock(), io::stdout(), io::stderr(), false);
}

fn main_loop<R, W, E>(input: R, output: W, error: E, testing: bool) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let mut context = RotfContext::default_context(input, output, error, testing);

  credits::welcome(&mut context);

  loop {
    context.println("-------------------");
    context.println("");
    context.print(" > ");
    match context.read_line() {
      Ok(cmd) => {
        context.println("");
        commands::parse_command(&cmd, &mut context);
      },
      Err(e) => {
        context.print_error("reading command", &e);
        context.println("");
      },
    }
    if context.exit {
      break;
    }
  }
}


#[cfg(test)]
pub mod test_main {
  use std::str;
  use std::io::{BufRead, Write};
  use crate::{main_loop, commands::parse_command, context::{RotfContext, test_context::TestContext}};

  pub fn run_main_loop(mut input: Vec<&str>) -> (String, String) {
    input.push("exit");
    let binding = input.join("\n");
    let input_stream = binding.as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    main_loop(&input_stream[..], &mut output, &mut error, false);
    let binding = output.clone();
    let output_str = str::from_utf8(&binding).unwrap();
    let binding = error.clone();
    let error_str = str::from_utf8(&binding).unwrap();
    return (output_str.to_string(), error_str.to_string());
  }

  pub fn run_cmd_output(cmd: &str) -> (String, String) {
    let input = "".as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    parse_command(cmd, &mut context);
    let binding = output.clone();
    let output_str = str::from_utf8(&binding).unwrap();
    let binding = error.clone();
    let error_str = str::from_utf8(&binding).unwrap();
    return (output_str.to_string(), error_str.to_string());
  }

  pub fn run_cmd_input(cmd: &str, extra_input: &str) -> (String, String) {
    let input = extra_input.as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    parse_command(cmd, &mut context);
    let binding = output.clone();
    let output_str = str::from_utf8(&binding).unwrap();
    let binding = error.clone();
    let error_str = str::from_utf8(&binding).unwrap();
    return (output_str.to_string(), error_str.to_string());
  }

  pub fn run_cmd_context(cmd: &str) -> TestContext {
    let input = "".as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    parse_command(cmd, &mut context);
    return TestContext::new(context);
  }

  pub fn run_cmd_context_input(cmd: &str, extra_input: &str) -> TestContext {
    let input = extra_input.as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    parse_command(cmd, &mut context);
    return TestContext::new(context);
  }

  pub fn run_cmd<R, W, E>(cmd: &str, context: &mut RotfContext<R, W, E>) where
    R: BufRead,
    W: Write,
    E: Write,
  {
    parse_command(cmd, context);
  }

  #[test]
  fn it_should_run_main_loop() {
    let (output, error) = run_main_loop(vec![""]);
    assert!(output.contains("Welcome to 'Rise of the Frogs'"));
    assert_eq!(error, "");
  }

  #[test]
  fn it_should_set_last_cmd() {
    let context = run_cmd_context("a cmd with params");
    assert_eq!(context.last_cmd, "a");
    assert_eq!(context.last_params, "cmd with params");
  }
}