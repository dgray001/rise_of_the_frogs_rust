use crate::commands;
use crate::game::RotfGame;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
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
  pub output: W,
  error: E,
  pub exit: bool,

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
      self.print_error("Tried println", &e);
      return 0;
    });
  }

  pub fn print_data<D>(&mut self, text: &str, data: D) where
    D: Display
  {
    self.output.write(format!("{}: {}", text, data).as_bytes()).unwrap_or_else(|e| {
      self.print_error("Tried print_data", &e);
      return 0;
    });
  }

  pub fn eprintln(&mut self, text: &str) {
    self.output.write([text, "\n"].concat().as_bytes()).unwrap_or_default();
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
      exit: false,
      
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


#[cfg(test)]
pub mod test_context {
  use crate::{game::RotfGame, context::*};

  pub struct TestContext {
    pub exit: bool,
  
    pub context_state: ContextState,
    pub all_commands: HashMap<String, commands::Command>,
    pub commands: HashMap<String, commands::Command>,
    pub last_cmd: String,
    pub last_params: String,
  
    pub curr_game: Option<RotfGame>,
  }
  
  #[cfg(test)]
  impl TestContext {
    pub fn new<R, W, E>(context: RotfContext<R, W, E>) -> TestContext where
      R: BufRead,
      W: Write,
      E: Write,
    {
      return TestContext {
        exit: context.exit,
        
        context_state: context.context_state,
        all_commands: context.all_commands,
        commands: context.commands,
        last_cmd: context.last_cmd,
        last_params: context.last_params,
    
        curr_game: context.curr_game,
      };
    }
  }
}