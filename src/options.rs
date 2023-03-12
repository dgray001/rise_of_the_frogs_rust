use crate::filesystem;

use std::{io::BufRead, path::Path};

pub struct RotfOptions {
  pub sleep_factor: f64,
}

impl RotfOptions {
  pub fn default() -> RotfOptions {
    let mut options = RotfOptions {
      sleep_factor: 1.0,
    };
    match filesystem::open_file("data/saves/options.rotf".to_string()) {
      Ok(f) => {
        for oline in f.lines() {
          let line = oline.unwrap_or("".to_string());
          if !line.clone().contains(":") {
            continue;
          }
          let (key, mut value) = line.split_once(":").unwrap();
          value = value.trim();
          match key.trim() {
            "sleep_factor" => options.sleep_factor = value.parse::<f64>().unwrap_or(1.0),
            _ => {},
          }
        }
      },
      Err(_e) => {
        if !Path::new("data/saves/options.rotf").exists() {
          options.save();
        }
      },
    }
    return options;
  }

  pub fn save(&self) {
    match filesystem::create_file("data/saves/options.rotf".to_string(), self.file_content()) {
      _ => {},
    }
  }

  pub fn file_content(&self) -> String {
    let mut contents = String::new();
    contents += &format!("\nsleep_factor: {}", self.sleep_factor.clone());
    return contents;
  }
}