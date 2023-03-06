use crate::context::RotfContext;
use crate::commands::Command;

pub fn ls(context: &RotfContext) {
  println!("Context Commands:");
  list_commands(Command::context_state_commands(context.context_state.clone()));
  println!("\nSystem Commands:");
  list_commands(Command::system_commands());
}

fn list_commands(cmds: Vec<Command>) {
  for cmd in cmds {
    if cmd.aliases().is_empty() {
      println!("  {}: {}", cmd.name(), cmd.description());
    } else {
      println!("  {} (aliases: {}): {}", cmd.name(), cmd.aliases().join(", "), cmd.description());
    }
  }
}

pub fn help(context: &RotfContext) {
  let commands = context.commands.clone();
  if context.last_params.clone().is_empty() {
    println!("Below is general information; if you want information about a specific command use 'help {{cmd}}'");
    println!();
    println!("You interact with the program by typing a command");
    println!("Use the 'ls' command to see the current list of commands you can use");
    println!("No command is case-sensitive, so 'LS' is the same as 'ls'");
    return;
  }
  match commands.get(&context.last_params) {
    Some(cmd) => cmd.helptext(),
    None => {
      eprintln!("{} is not a recognized command", context.last_params);
      println!();
      println!("If you want general help text, use 'help'");
    },
  }
}

pub fn exit() {
  std::process::exit(0);
}