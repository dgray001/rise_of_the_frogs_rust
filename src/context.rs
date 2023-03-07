use crate::commands;
use crate::game::RotfGame;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, Write, self};

#[derive(Clone)]
pub enum ContextState {
  HOME,
  INGAME,
}

pub struct RotfContext<R, W, E> where
  R: BufRead,
  W: Write,
  E: Write,
{
  input: R,
  output: W,
  error: E,

  pub context_state: ContextState,
  pub all_commands: HashMap<String, commands::Command>,
  pub commands: HashMap<String, commands::Command>,
  pub last_cmd: String,
  pub last_params: String,

  pub curr_game: Option<RotfGame>,
}

impl<R, W, E> RotfContext<R, W, E> where
  R: BufRead,
  W: Write,
  E: Write,
{
  pub fn print(&mut self, text: &str) {
    self.output.write(text.as_bytes()).unwrap_or_else(|e| {
      self.print_error("Tried print", &e);
      return 0;
    });
    self.output.flush().unwrap_or_else(|e| {
      self.print_error("Tried print", &e);
    });
  }

  pub fn println(&mut self, text: &str) {
    self.output.write([text, "\n"].concat().as_bytes()).unwrap_or_else(|e| {
      self.print_error("Tried print", &e);
      return 0;
    });
  }

  pub fn print_error(&mut self, attempt: &str, e: &dyn Error) {
    self.error.write(format!("Error {}: {}", attempt, e).as_bytes()).unwrap_or_default();
  }

  pub fn read_line(&mut self) -> Result<String, io::Error> {
    let mut cmd = String::new();
    match self.input.read_line(&mut cmd) {
      Ok(_n) => Ok(cmd),
      Err(e) => Err(e)
    }
  }

  pub fn default_context(input: R, output: W, error: E) -> RotfContext<R, W, E> {
    let mut context: RotfContext<R, W, E> = RotfContext {
      input,
      output,
      error,
      
      context_state: ContextState::HOME,
      all_commands: commands::get_all_commands(),
      commands: HashMap::new(),
      last_cmd: "".to_string(),
      last_params: "".to_string(),
  
      curr_game: None,
    };
    context.commands = commands::get_current_commands(&context);
    return context;
  }
}