use crate::context::RotfContext;

pub fn ls(context: &RotfContext) {
  for cmd in context.commands.values() {
    println!("{}: {}", cmd.name, cmd.description);
  }
}

pub fn help(_context: &RotfContext) {
}

pub fn exit(_context: &RotfContext) {
  std::process::exit(0);
}