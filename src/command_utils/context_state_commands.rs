use crate::{filesystem, context::{self, RotfContext, ContextState}, game::{self, RotfDifficulty}};

use std::{io::{Write, Error, BufRead}, path::PathBuf, ffi::OsStr};


// Launch a saved or new game
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


// Delete a saved game
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
        let delete_name = str::replace(&context.last_params, " ", "_");
        if save_name == delete_name {
          filesystem::delete_folder(format!("data/saves/{}", delete_name)).unwrap_or_else(|e| {
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


// Display info on current player
pub fn me<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match &context.curr_game {
    Some(game) => {
      let name = game.name.clone();
      let player = game.player.me(name);
      context.println(&player);
    }
    None => {
      context.eprintln("Can't use ME when there's no game");
      return;
    },
  }
}


// Saves game and returns to main menu
pub fn save<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match &context.curr_game {
    Some(game) => {
      match game.save() {
        Ok(()) => {
          context.curr_game = None;
          context.context_state = ContextState::HOME;
          context.println("Saved game");
        },
        Err(e) => {
          context.print_error("save game", &e);
        },
      }
    }
    None => {
      context.eprintln("Can't use SAVE when there's no game");
      return;
    },
  }
}


// Helper functions
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
  let invalid_characters = "-<>:\"\\/|?*^";
  for char in invalid_characters.chars() {
    if name.contains(char) {
      context.println(format!("Cannot use the following characters: {}", invalid_characters).as_str());
      return;
    }
  }
  let invalid_names = vec!["com1", "com2", "com3", "com4", "com5", "com6",
    "com7", "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7",
    "lpt8", "lpt9", "con", "nul", "prn"];
  for invalid_name in invalid_names {
    if name == invalid_name {
      context.println(format!("Cannot use the following name: {}", invalid_name).as_str());
      return;
    }
  }
  if name.len() > 30 {
    context.println("Cannot enter a name of more than 30 characters");
    return;
  }
  if name.starts_with(".") || name.ends_with(".") {
    context.println("Cannot start or end name with a '.'");
    return;
  }
  let mut current_games = Vec::new();
  match filesystem::open_folder_or_create("data/saves".to_owned()) {
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
          match game::RotfGame::load(save_name) {
            Ok(game) => {
              context.curr_game = Some(game);
              context.context_state = ContextState::INGAME;
              context.println("Launching game\n");
              me(context);
            },
            Err(e) => {
              context.print_error("loading game", &e);
            },
          }
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
  match filesystem::open_folder_or_create("data/saves".to_owned()) {
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
  use crate::{test_main::*, commands::{context_state_commands::*, get_current_commands}, game::RotfGame, context::ContextState};

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
    let context = run_cmd_context_input("launch new", "test new context\n3\n");
    assert!(Path::new("data/saves/test_new_context").exists());
    assert!(get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "test_new_context"));
    let game = context.curr_game.unwrap();
    assert_eq!(game.name, "test new context");
    assert_eq!(game.difficulty, RotfDifficulty::HARD);
    let saved_game = RotfGame::load("test new_context".to_owned()).unwrap();
    assert_eq!(saved_game.name, "test new context");
    assert_eq!(saved_game.difficulty, RotfDifficulty::HARD);
    run_cmd_output("delete test_new_context"); // clean up test
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
    assert_eq!(context.context_state, ContextState::INGAME);
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
    run_cmd_input("launch new", "deletable with spaces\n2\n");
    assert!(get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "deletable_with_spaces"));
    let (output, error) = run_cmd_output("delete deletable with_spaces");
    assert!(!get_saved_games().unwrap().iter().any(|p| p.file_name().unwrap() == "deletable_with_spaces"));
    assert_eq!(output, "");
    assert_eq!(error, "");
  }

  #[test]
  fn test_me_when_home() {
    let (output, error) = run_cmd_output("me");
    assert!(output.contains("Invalid command"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_me() {
    let input = "".as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    context.context_state = ContextState::INGAME;
    context.commands = get_current_commands(&mut context);
    context.curr_game = Some(RotfGame::new("test me".to_owned(), RotfDifficulty::default()));

    run_cmd("me", &mut context);

    let output = std::str::from_utf8(&output).unwrap();
    let error = std::str::from_utf8(&error).unwrap();
    assert!(output.contains("Player Info"));
    assert!(output.contains("Name: test me"));
    assert_eq!(error, "");
    run_cmd_output("delete test me"); // clean up test
  }

  #[test]
  fn test_save_when_home() {
    let (output, error) = run_cmd_output("save");
    assert!(output.contains("Invalid command"));
    assert_eq!(error, "");
  }

  #[test]
  fn test_save() {
    let input = "".as_bytes();
    let mut output = Vec::new();
    let mut error = Vec::new();
    let mut context = RotfContext::default(&input[..], &mut output, &mut error);
    context.context_state = ContextState::INGAME;
    context.commands = get_current_commands(&mut context);
    context.curr_game = Some(RotfGame::new("test save".to_owned(), RotfDifficulty::default()));

    run_cmd("save", &mut context);

    assert_eq!(context.context_state, ContextState::HOME);
    assert!(context.curr_game.is_none());
    let output = std::str::from_utf8(&output).unwrap();
    let error = std::str::from_utf8(&error).unwrap();
    run_cmd_output("delete test save"); // clean up test
    assert!(output.contains("Saved game"));
    assert_eq!(error, "");
  }
}
