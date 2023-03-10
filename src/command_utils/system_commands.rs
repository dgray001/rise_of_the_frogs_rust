use std::io::{Write, BufRead};

use crate::context::RotfContext;
use crate::commands::Command;

pub fn ls<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println("Context Commands:");
  list_commands(Command::context_state_commands(context.context_state.clone()), context);
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


#[cfg(test)]
pub mod test_system_commands {
  use crate::test_main::{run_cmd_output, run_cmd_context};

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
}