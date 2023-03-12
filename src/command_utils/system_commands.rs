use std::io::{Write, BufRead};

use crate::context::RotfContext;
use crate::commands::Command;
use crate::cutscene::RotfCutscene;

pub fn ls<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println("Context Commands:");
  list_commands(Command::context_state_commands(context), context);
  context.println("\nSystem Commands:");
  list_commands(Command::system_commands(), context);
}

fn list_commands<R, W, E>(cmds: Vec<Command>, context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  for cmd in cmds {
    if cmd.aliases().is_empty() {
      context.println(&format!("  {}: {}", cmd.name(), cmd.description()));
    } else {
      context.println(&format!("  {} (aliases: {}): {}",
        cmd.name(), cmd.aliases().join(", "), cmd.description()));
    }
  }
}

pub fn help<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let commands = context.commands.clone();
  if context.last_params.clone().is_empty() {
    context.println("Below is general information; if you want information about a specific command use 'help {{cmd}}'");
    context.println("");
    context.println("You interact with the program by typing a command");
    context.println("Use the 'ls' command to see the current list of commands you can use");
    context.println("No command is case-sensitive, so 'LS' is the same as 'ls'");
    return;
  }
  match commands.get(&context.last_params) {
    Some(cmd) => cmd.helptext(context),
    None => {
      context.eprintln(&format!("{} is not a recognized command", context.last_params));
      context.println("");
      context.println("If you want general help text, use 'help'");
    },
  }
}

pub fn replay<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match context.last_params.trim() {
    "cutscene" => {
      let mut cutscene = RotfCutscene::LAUNCH_GAME;
      let mut play_cutscene = false;
      match &context.curr_game {
        Some(game) => {
          play_cutscene = true;
          cutscene = game.last_cutscene.clone();
        },
        None => {},
      }
      if play_cutscene {
        match cutscene.play(context) {
          Ok(_) => {},
          Err(e) => {
            context.print_error("playing cutscene", &e);
          }
        };
      }
    }
    _ => {
      context.println("Unrecognized replay parameter");
      context.println("Use 'replay cutscene' to play the last cutscene");
    },
  }
}

pub fn options<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println("  -- Options Menu --");
  context.println("To modify an option, enter its associated number; to leave, enter '0'");
  loop {
    context.println("");
    context.println(format!("1: sleep_factor, value: {}", context.options.sleep_factor).as_str());
    context.println("");
    context.print(" choose an option > ");
    match context.read_line() {
      Ok(input) => {
        match input.trim() {
          "0" => return,
          "1" => {
            context.println("sleep factor");
            context.println(format!("  current value: {}", context.options.sleep_factor).as_str());
            context.println("  accepted values: 0-2");
            context.println("");
            context.print(" enter new value > ");
            match context.read_line() {
              Ok(v) => {
                let new_val = v.trim().parse::<f64>().unwrap_or(-1.0);
                if new_val < 0.0 || new_val > 2.0 {
                  context.println("Not an accepted value for sleep factor");
                }
                else {
                  context.options.sleep_factor = new_val;
                  context.options.save();
                  context.println(format!("Changed sleep factor to {}", new_val).as_str());
                }
              }
              Err(e) => context.print_error("reading input", &e),
            }
          }
          _ => context.println("Invalid input. If you wish to leave the option menu, enter '0'"),
        }
      },
      Err(e) => context.print_error("reading input", &e),
    }
  }
}


#[cfg(test)]
pub mod test_system_commands {
  use rstest::*;
  use crate::test_main::*;
  use crate::context::RotfContext;
  use crate::game::{RotfGame, RotfDifficulty};

  #[test]
  fn test_ls() {
    let (output, error) = run_cmd_output("ls");
    assert!(output.contains("Context Commands:"));
    assert!(output.contains("System Commands:"));
    assert!(output.contains("ls (aliases: list): List available commands"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_ls_alias() {
    let (output1, error1) = run_cmd_output("ls");
    let (output2, error2) = run_cmd_output("list");
    assert_eq!(output1, output2);
    assert_eq!(error1, error2);
  }

  #[test]
  fn test_help() {
    let (output, error) = run_cmd_output("help");
    assert!(output.contains("Below is general information; if you want information about a specific command use 'help {{cmd}}'"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_help_param() {
    let (output, error) = run_cmd_output("help ls");
    assert!(output.contains("Lists all available commands"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_help_unknown_param() {
    let (output, error) = run_cmd_output("help unknown");
    assert!(output.contains("unknown is not a recognized command"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_exit() {
    let context = run_cmd_context("exit");
    assert_eq!(context.exit, true);
  }

  #[test]
  fn test_credits() {
    let (output, error) = run_cmd_output("credits");
    assert!(output.contains("Created by Daniel Gray"));
    assert_eq!(error, "");
  }

  #[rstest]
  #[case::replay("replay", "Unrecognized replay parameter")]
  #[case::unknown("replay unknown", "Unrecognized replay parameter")]
  #[case::cutscene("replay cutscene", "+++++ Chapter 1: A Legend is Laid +++++")]
  fn test_replay(#[case] cmd: &str, #[case] expected: &str) {
    let input = "".as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    let game = RotfGame::new(format!("test replay {}", cmd), RotfDifficulty::default());
    context.curr_game = Some(game);

    run_cmd(cmd, &mut context);

    let output = std::str::from_utf8(&output).unwrap();
    let error = std::str::from_utf8(&error).unwrap();
    run_cmd_output(format!("delete test replay {}", cmd).as_str()); // clean up test
    assert!(output.contains(expected));
    assert_eq!(error, "");
  }
}