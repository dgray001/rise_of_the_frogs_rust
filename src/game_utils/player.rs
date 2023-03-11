use crate::commands::Command;


pub struct RotfPlayer {
  pub level: u8,
}

impl RotfPlayer {
  pub fn new() -> RotfPlayer {
    return RotfPlayer {
      level: 0,
    }
  }

  pub fn environment_commands(&self) -> Vec<Command> {
    return vec![];
  }

  fn tier(&self) -> u8 {
    return 1 + self.level / 10;
  }

  pub fn me(&self, name: String) -> String {
    let mut str = String::new();
    str += "Player Info";
    str += &format!("\n   Name: {}", name);
    str += &format!("\n  Level: {}", self.level);
    str += &format!("\n   Tier: {}", self.tier());
    return str;
  }
}