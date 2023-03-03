use crate::context::RotfContext;

pub fn ls(context: &RotfContext) {
  for cmd in context.commands.values() {
    if cmd.aliases().is_empty() {
      println!("{}: {}", cmd.name(), cmd.description());
    } else {
      println!("{} (aliases: {}): {}", cmd.name(), cmd.aliases().join(", "), cmd.description());
    }
  }
}

pub fn help(_context: &RotfContext) {
  println!("Below is general information; if you want information about a specific command use 'help cmd'");
  println!();
  println!("You interact with the program by typing a command");
  println!("Use the 'ls' command to see the current list of commands you can use");
  println!("No command is case-sensitive, so 'LS' is the same as 'ls'");
}

pub fn exit(_context: &RotfContext) {
  std::process::exit(0);
}