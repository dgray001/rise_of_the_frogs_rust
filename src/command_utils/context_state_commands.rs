use crate::filesystem::{open_folder_or_create, create_folder, delete_folder};

use std::{io::{self, Write, Error}, path::PathBuf, ffi::OsStr};

pub fn launch(param: &String) {
  if param.is_empty() {
    println!("Must specify an arg when using launch");
    println!("Use 'launch {{new}}' to launch a new game");
    println!("Use 'launch {{saved_game_name}}' to launch a saved game");
    println!("Use 'launch ls' to list the current saved games");
    return;
  }
  if param == "new" {
    launch_new();
    return;
  }
  if param == "ls" {
    launch_ls();
    return;
  }
  launch_load();
}

pub fn delete(param: &String) {
  if param.is_empty() {
    println!("Must specify a saved game to delete.");
    println!("You can view the current saved games with 'launch ls'.");
    return;
  }
  match get_saved_games() {
    Ok(entries) => {
      for entry in entries {
        let save_name = entry.file_name().unwrap_or_else(|| OsStr::new(""))
          .to_string_lossy().trim().to_lowercase();
        if &save_name == param {
          delete_folder(format!("data/saves/{}", param)).unwrap_or_else(|e| {
            println!("Error deleting saved game: {}", e);
          });
          return;
        }
      }
    },
    Err(e) => eprintln!("Error finding saved games: {}", e),
  }
  println!("Saved game doesn't exist.");
}


fn launch_new() {
  let mut name = String::new();
  print!("Save name: ");
  io::stdout().flush().unwrap();
  match io::stdin().read_line(&mut name) {
    Ok(_n) => {},
    Err(e) => {
      eprintln!("Error reading input: {}", e);
      return;
    },
  }
  let new_game = name.trim().to_lowercase();
  if new_game.is_empty() {
    println!("Can't enter empty name.");
    return;
  }
  let mut current_games = Vec::new();
  match open_folder_or_create("data/saves".to_string()) {
    Ok(games) => {
      for game in games.iter() {
        current_games.push(game.file_name().unwrap_or_else(|| OsStr::new(""))
          .to_string_lossy().trim().to_lowercase());
      }
    }
    Err(e) => {
      eprintln!("Error finding saved games: {}", e);
      return;
    }
  }
  if current_games.contains(&new_game) {
    println!("That saved game already exists.");
    println!("To delete it use 'delete {}'.", new_game);
    println!("To launch it use 'launch {}'.", new_game);
    return;
  }
  match create_folder(format!("data/saves/{}", new_game)) {
    Ok(()) => {},
    Err(e) => {
      println!("Error creating new game: {}", e);
      return;
    }
  }
  println!("add files");
}

fn launch_ls() {
  match get_saved_games() {
    Ok(entries) => {
      for entry in entries {
        let save_name = entry.file_name().unwrap_or_else(|| OsStr::new(""));
        println!("{}", save_name.to_string_lossy().to_lowercase());
      }
    },
    Err(e) => eprintln!("Error finding saved games: {}", e),
  }
}

fn launch_load() {
  println!("Try to launch game.");
}

fn get_saved_games() -> Result<Vec<PathBuf>, Error> {
  let mut saved_games = Vec::new();
  match open_folder_or_create("data/saves".to_string()) {
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