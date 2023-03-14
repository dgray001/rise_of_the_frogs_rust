use std::{io::{BufRead, Write, Error}, fmt, str::FromStr};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{context::{RotfContext, ContextState}, game::GameState, filesystem};


// CutsceneMode determines how cutscene is played
#[allow(non_camel_case_types)]
#[derive(Debug, EnumIter, PartialEq)]
enum CutsceneMode {
  INSTANT, // print line with no pause
  LETTER_BY_LETTER, // pauses after each letter
  LINE_AFTER, // print a new line then pause after each line
  LINE_BEFORE, // pause then print a new line after each line
}

impl fmt::Display for CutsceneMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for CutsceneMode {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for mode in CutsceneMode::iter() {
      if mode.to_string() == s {
        return Ok(mode);
      }
    }
    Err(())
  }
}

impl CutsceneMode {
  fn default() -> CutsceneMode {
    return CutsceneMode::LETTER_BY_LETTER;
  }
}


// RotfCutscene lists all possible cutscenes
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum RotfCutscene {
  LAUNCH_GAME,
}

impl fmt::Display for RotfCutscene {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl RotfCutscene {
  pub fn resolve_context<R, W, E>(context: &mut RotfContext<R, W, E>) where
    R: BufRead,
    W: Write,
    E: Write,
  {
    if context.context_state != ContextState::INGAME {
      return;
    }
    let mut cutscene = RotfCutscene::LAUNCH_GAME;
    let mut play_cutscene = false;
    match context.curr_game.as_mut() {
      Some(game) => {
        if game.state == GameState::CUTSCENE {
          game.state = GameState::ENVIRONMENT;
          play_cutscene = true;
          cutscene = game.last_cutscene.clone();
        }
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

  pub fn play<R, W, E>(&self, context: &mut RotfContext<R, W, E>) -> Result<(), Error> where
    R: BufRead,
    W: Write,
    E: Write,
  {
    context.lnprint_sleep("");
    let mut mode = CutsceneMode::default();
    for oline in filesystem::open_file(format!("data/cutscenes/{}.rotf", self))?.lines() {
      let line = oline?;
      if line.trim().starts_with("%%% mode =") {
        mode = CutsceneMode::from_str(line.split_once("=")
          .unwrap_or(("", "")).1.trim()).unwrap_or(mode);
        continue;
      }
      match mode {
        CutsceneMode::INSTANT => {
          context.println(line.as_str());
        },
        CutsceneMode::LETTER_BY_LETTER => {
          let mut sublines = line.split("%%").peekable();
          while let Some(subline) = sublines.next() {
            if sublines.peek().is_none() {
              context.println_letter_by_letter(subline);
            }
            else {
              context.print_letter_by_letter(subline);
            }
          }
        },
        CutsceneMode::LINE_AFTER => {
          let mut sublines = line.split("%%").peekable();
          while let Some(subline) = sublines.next() {
            if sublines.peek().is_none() {
              context.println_sleep(subline);
            }
            else {
              context.print_sleep(subline);
            }
          }
        },
        CutsceneMode::LINE_BEFORE => {
          for subline in line.split("%%") {
            context.print_sleep(subline);
          }
          context.println("");
        },
      }
    }
    context.println_sleep("");
    Ok(())
  }
}
