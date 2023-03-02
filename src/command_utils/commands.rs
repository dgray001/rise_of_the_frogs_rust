mod system_commands;

use std::collections::HashMap;

use crate::context::RotfContext;

pub fn parse_command(name: &str, context: &mut RotfContext) {
  match context.commands.get(&String::from(name.trim())) {
    Some(cmd) => (cmd.callback)(context),
    None => println!("Invalid command."),
  }
  context.commands = get_current_commands();
}

pub struct Command<'a> {
  name: &'a str,
  description: &'a str,
  callback: Box<dyn Fn(&RotfContext)>,
}

pub fn get_current_commands() -> HashMap<String, Command<'static>> {
  let mut cmds = HashMap::new();
  for cmd in get_system_commands() {
    cmds.insert(String::from(cmd.name), cmd);
  }
  return cmds;
}

fn get_system_commands() -> Vec<Command<'static>> {
  let mut cmds = Vec::new();
  cmds.push(Command {
    name: "ls",
    description: "List available commands",
    callback: Box::new(system_commands::ls),
  });
  cmds.push(Command {
    name: "help",
    description: "List available commands",
    callback: Box::new(system_commands::help),
  });
  cmds.push(Command {
    name: "exit",
    description: "List available commands",
    callback: Box::new(system_commands::exit),
  });
  return cmds;
}