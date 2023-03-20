use crate::commands;
use crate::game::RotfGame;
use crate::options::{self, RotfOptions};

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::io::{BufRead, Write, self};
use std::thread;
use std::time::Duration;

use self::unit_loader::UnitLoader;

pub mod unit_loader;
pub mod constants;


// Overall program state
#[derive(Clone, Debug, PartialEq)]
pub enum ContextState {
  HOME,
  INGAME,
}


// Context object
pub struct RotfContext<R, W, E> where
  R: BufRead,
  W: Write,
  E: Write,
{
  pub input: R,
  pub output: W,
  pub error: E,
  testing: bool,
  pub exit: bool,

  pub options: options::RotfOptions,
  pub context_state: ContextState,
  pub all_commands: HashMap<String, commands::Command>,
  pub commands: HashMap<String, commands::Command>,
  pub last_cmd: String,
  pub last_params: String,

  pub curr_game: Option<RotfGame>,
  pub unit_loader: unit_loader::UnitLoader,
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

  pub fn lnprint(&mut self, text: &str) {
    self.output.write(["\n", text].concat().as_bytes()).unwrap_or_else(|e| {
      self.print_error("Tried lnprint", &e);
      return 0;
    });
    self.output.flush().unwrap_or_else(|e| {
      self.print_error("Tried lnprint", &e);
    });
  }

  pub fn print_sleep(&mut self, text: &str) {
    self.print(text);
    self.sleep_line();
  }

  pub fn println_sleep(&mut self, text: &str) {
    self.println(text);
    self.sleep_line();
  }

  pub fn lnprint_sleep(&mut self, text: &str) {
    self.lnprint(text);
    self.sleep_line();
  }

  pub fn print_letter_by_letter(&mut self, text: &str) {
    for char in text.chars() {
      self.print(char.to_string().as_str());
      self.sleep_char();
    }
    self.sleep_line();
  }

  pub fn println_letter_by_letter(&mut self, text: &str) {
    for char in text.chars() {
      self.print(char.to_string().as_str());
      self.sleep_char();
    }
    self.sleep_line();
    self.println("");
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
    self.error.write(format!("Error {}: {}\n", attempt, e).as_bytes()).unwrap_or_default();
  }

  pub fn read_line(&mut self) -> Result<String, io::Error> {
    let mut cmd = String::new();
    match self.input.read_line(&mut cmd) {
      Ok(_n) => Ok(cmd),
      Err(e) => Err(e)
    }
  }

  fn sleep_line(&self) {
    self.sleep_amount((3000.0 * self.options.sleep_factor).round() as u64);
  }

  fn sleep_char(&self) {
    self.sleep_amount((6.0 * self.options.sleep_factor).round() as u64);
  }

  fn sleep_amount(&self, amount: u64) {
    if self.testing {
      return;
    }
    thread::sleep(Duration::from_millis(amount));
  }

  pub fn default(input: R, output: W, error: E) -> RotfContext<R, W, E> {
    let mut context: RotfContext<R, W, E> = RotfContext {
      input,
      output,
      error,
      testing: true, // have to manually override
      exit: false,
      options: RotfOptions::default(),
      
      context_state: ContextState::HOME,
      all_commands: commands::get_all_commands(),
      commands: HashMap::new(),
      last_cmd: "".to_owned(),
      last_params: "".to_owned(),
  
      curr_game: None,
      unit_loader: UnitLoader::new(), // empty loader
    };
    context.commands = commands::get_current_commands(&mut context);
    return context;
  }

  pub fn default_context(input: R, output: W, error: E, testing: bool) -> RotfContext<R, W, E> {
    let mut context = RotfContext::default(input, output, error);
    context.testing = testing;
    context.commands = commands::get_current_commands(&mut context);
    return context;
  }

  pub fn launch_game(&mut self, game: RotfGame, new: bool) {
    if new {
      self.println("Launching new game ...\n");
    }
    else {
      self.println("Launching game ...\n");
    }
    // load unit data
    match self.unit_loader.load_data() {
      Ok(()) => {},
      Err(e) => {
        self.print_error("loading unit data", &e);
        return;
      },
    }
    self.unit_loader.update_current_units(&game.player);
    // load item data
    // launch game
    self.curr_game = Some(game);
    self.context_state = ContextState::INGAME;
  }
}


// Testing
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
