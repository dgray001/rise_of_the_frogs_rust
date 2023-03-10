use crate::{filesystem, context::{self, RotfContext}, game};

use std::{io::{self, Write, Error, BufRead}, path::PathBuf, ffi::OsStr};

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
    context.println("Must specify a saved game to delete.");
    context.println("You can view the current saved games with 'launch ls'.");
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
  context.println("Saved game doesn't exist.");
}


fn launch_new<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let mut name = String::new();
  context.print("Save name: ");
  match io::stdin().read_line(&mut name) {
    Ok(_n) => {},
    Err(e) => {
      context.print_error("reading input", &e);
      return;
    },
  }
  name = name.trim().to_lowercase();
  if name.is_empty() {
    context.println("Can't enter empty name.");
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
    context.println("That saved game already exists.");
    context.println(&format!("To delete it use 'delete {}'.", name));
    context.println(&format!("To launch it use 'launch {}'.", name));
    return;
  }
  let new_game = game::RotfGame::new(name.clone());
  match new_game.save() {
    Ok(()) => {},
    Err(e) => {
      context.print_error("creating new game", &e);
      return;
    }
  }
  context.curr_game = Some(new_game);
  context.context_state = context::ContextState::INGAME;
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
  context.println("Try to launch game.");
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