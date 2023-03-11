use crate::{filesystem, context::{self, RotfContext}, game::{self, RotfDifficulty}};

use std::{io::{Write, Error, BufRead}, path::PathBuf, ffi::OsStr};

pub fn launch<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  if context.last_params.is_empty() {
    context.println("Must specify an arg when using launch");
    context.println("Use 'launch {{new}}' to launch a new game");
    context.println("Use 'launch {{saved_game_name}}' to launch a saved game");
    context.println("Use 'launch ls' to list the current saved games");
    return;
  }
  if context.last_params == "new" {
    launch_new(context);
    return;
  }
  if context.last_params == "ls" {
    launch_ls(context);
    return;
  }
  launch_load(context);
}

pub fn delete<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  if context.last_params.is_empty() {
    context.println("Must specify a saved game to delete");
    context.println("You can view the current saved games with 'launch ls'");
    return;
  }
  match get_saved_games() {
    Ok(entries) => {
      for entry in entries {
        let save_name = entry.file_name().unwrap_or_else(|| OsStr::new(""))
          .to_string_lossy().trim().to_lowercase();
        if &save_name == &context.last_params {
          filesystem::delete_folder(format!("data/saves/{}", context.last_params)).unwrap_or_else(|e| {
            context.print_error("deleting saved game", &e);
          });
          return;
        }
      }
    },
    Err(e) => context.print_error("finding saved games", &e),
  }
  context.println("Saved game doesn't exist");
}


fn launch_new<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.print("Enter a name: ");
  let mut name;
  match context.read_line() {
    Ok(n) => name = n,
    Err(e) => {
      context.print_error("reading input", &e);
      return;
    },
  }
  name = name.trim().to_lowercase();
  if name.is_empty() {
    context.println("Can't enter empty name");
    return;
  }
  let mut current_games = Vec::new();
  match filesystem::open_folder_or_create("data/saves".to_string()) {
    Ok(games) => {
      for game in games.iter() {
        current_games.push(game.file_name().unwrap_or_else(|| OsStr::new(""))
          .to_string_lossy().trim().to_lowercase());
      }
    }
    Err(e) => {
      context.print_error("Error finding saved games: {}", &e);
      return;
    }
  }
  if current_games.contains(&name) {
    context.println("That saved game already exists");
    context.println(&format!("To delete it use 'delete {}'.", name));
    context.println(&format!("To launch it use 'launch {}'.", name));
    return;
  }
  let difficulty;
  match choose_difficulty(context) {
    Ok(dif) => difficulty = dif,
    Err(e) => {
      context.print_error("reading input", &e);
      return;
    }
  }
  let new_game = game::RotfGame::new(name.clone(), difficulty);
  match new_game.save() {
    Ok(()) => {},
    Err(e) => {
      context.print_error("creating new game", &e);
      return;
    }
  }
  context.curr_game = Some(new_game);
  context.context_state = context::ContextState::INGAME;
  context.println("\nLaunching new game ...");
}

fn choose_difficulty<R, W, E>(context: &mut context::RotfContext<R, W, E>) -> Result<RotfDifficulty, Error> where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println("\nChoose a difficulty where:");
  context.println("  1: Easy");
  context.println("  2: Normal");
  context.println("  3: Hard");
  loop {
    context.print(" > ");
    match context.read_line() {
      Ok(input) => {
        match input.trim() {
          "1" => return Ok(RotfDifficulty::EASY),
          "2" => return Ok(RotfDifficulty::NORMAL),
          "3" => return Ok(RotfDifficulty::HARD),
          _ => context.println("Please enter a number from 1 to 3")
        }
      },
      Err(e) => {
        context.print_error("reading input", &e);
        return Err(e);
      },
    }
  }
}

fn launch_ls<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match get_saved_games() {
    Ok(entries) => {
      for entry in entries {
        let save_name = entry.file_name().unwrap_or_else(|| OsStr::new(""));
        context.println(&format!("{}", save_name.to_string_lossy().to_lowercase()));
      }
    },
    Err(e) => context.print_error("Error finding saved games: {}", &e),
  }
}

fn launch_load<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match get_saved_games() {
    Ok(entries) => {
      for entry in entries {
        let save_name = entry.file_name().unwrap_or_else(|| OsStr::new(""))
          .to_string_lossy().trim().to_lowercase();
        if &save_name == &context.last_params {
          context.curr_game = game::RotfGame::load(save_name).ok();
          return;
        }
      }
    },
    Err(e) => context.print_error("finding saved games", &e),
  }
  context.println("Saved game doesn't exist");
}

fn get_saved_games() -> Result<Vec<PathBuf>, Error> {
  let mut saved_games = Vec::new();
  match filesystem::open_folder_or_create("data/saves".to_string()) {
    Ok(entries) => {
      for entry in entries {
        if entry.is_dir() {
          saved_games.push(entry);
        }
      }
      Ok(saved_games)
    },
    Err(e) => Err(e),
  }
}


#[cfg(test)]
pub mod test_context_state_commands {
  use std::path::Path;
  use crate::{test_main::*, commands::context_state_commands::*};

  #[test]
  fn test_launch() {
    let (output, error) = run_cmd_output("launch");
    assert!(output.contains("Must specify an arg when using launch"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_launch_ls() {
    let (output, error) = run_cmd_output("launch ls");
    assert!(output.contains("test"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_launch_unknown() {
    let (output, error) = run_cmd_output("launch unknown");
    assert!(output.contains("Saved game doesn't exist"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_launch_new() {
    let (output, error) = run_cmd_input("launch new", "test_new\n1\n");
    assert!(output.contains("Enter a name:"));
    assert!(Path::new("data/saves/test_new").exists());
    assert!(get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "test_new"));
    assert_eq!(error, "");
    run_cmd_output("delete test_new"); // clean up test
  }

  #[test]
  fn test_launch_new_context() {
    let context = run_cmd_context_input("launch new", "test_new_context\n2\n");
    assert!(Path::new("data/saves/test_new_context").exists());
    assert!(get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "test_new_context"));
    run_cmd_output("delete test_new_context"); // clean up test
    assert_eq!(context.curr_game.unwrap().name, "test_new_context");
  }

  #[test]
  fn test_launch_new_exists() {
    assert!(Path::new("data/saves/test").exists());
    let (output, error) = run_cmd_input("launch new", "test\n");
    assert!(output.contains("Enter a name:"));
    assert!(output.contains("That saved game already exists"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_launch_game() {
    let context = run_cmd_context("launch test");
    assert_eq!(context.curr_game.unwrap().name, "test");
  }

  #[test]
  fn test_delete() {
    let (output, error) = run_cmd_output("delete");
    assert!(output.contains("Must specify a saved game to delete"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_delete_unknown() {
    let (output, error) = run_cmd_output("delete unknown");
    assert!(output.contains("Saved game doesn't exist"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_delete_game() {
    run_cmd_input("launch new", "deletable\n2\n");
    assert!(get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "deletable"));
    let (output, error) = run_cmd_output("delete deletable");
    assert!(!get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "deletable"));
    assert_eq!(output, "");
    assert_eq!(error, "");
  }
}