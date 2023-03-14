use std::io::{BufRead, Write};

use crate::context;
use crate::game::GameState;


// Ensures command is a valid environment command and context can take one
pub fn command<R, W, E>(context: &mut context::RotfContext<R, W, E>, cmd: &str) where
  R: BufRead,
  W: Write,
  E: Write,
{
  match context.curr_game.as_mut() {
    Some(game) => {
      if game.state == GameState::ENVIRONMENT {
        match cmd {
          "view" => view(context),
          "fight" => fight(context),
          "pickup" => pickup(context),
          _ => context.eprintln(format!("Environment command {} not implemented", cmd).as_str()),
        }
      }
      else {
        context.eprintln(format!("Cannot use environment command {} when not in environment", cmd).as_str());
      }
    },
    None => {
      context.eprintln(format!("Cannot use environment command {} with no game", cmd).as_str());
    },
  }
}


// These private functions it is safe to unwrap game and assume it is in an environment state
fn view<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let game = context.curr_game.as_mut().unwrap();
  let mut output_str = String::new();
  let mut index = 1;
  for unit in game.environment.units.iter_mut() {
    if !game.player.can_view(unit) {
      unit.view_index = -1;
      continue;
    }
    unit.view_index = index;
    index += 1;
    output_str += unit.to_string().as_str();
  }
  index = 1;
  for item in game.environment.items.iter_mut() {
    if !game.player.can_view(item) {
      item.view_index = -1;
      continue;
    }
    item.view_index = index;
    index += 1;
    output_str += item.to_string().as_str();
  }
  context.println(output_str.as_str());
}

fn fight<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let index = context.last_params.parse::<i64>().unwrap_or(-1);
  if index < 1 {
    context.println("Invalid index. Must be a positive integer.");
    return;
  }
  let game = context.curr_game.as_mut().unwrap();
  let mut fight_unit = None;
  for unit in game.environment.units.iter() {
    if unit.view_index != index {
      continue;
    }
    fight_unit = Some(unit);
  }
  match fight_unit {
    Some(unit) => {
      if !game.player.can_view(unit) {
        context.println("Unit no longer in view. Use 'view' to update view");
        return;
      }
      context.println("FIGHT");
    },
    None => {
      context.println("Unit not found")
    },
  }
}

fn pickup<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let index = context.last_params.parse::<i64>().unwrap_or(-1);
  if index < 1 {
    context.println("Invalid index. Must be a positive integer.");
    return;
  }
  let game = context.curr_game.as_mut().unwrap();
  let mut pickup_item = None;
  for item in game.environment.items.iter() {
    if item.view_index != index {
      continue;
    }
    pickup_item = Some(item);
  }
  match pickup_item {
    Some(item) => {
      if !game.player.can_view(item) {
        context.println("Item no longer in view. Use 'view' to update view");
        return;
      }
      context.println("FIGHT");
    },
    None => {
      context.println("Item not found")
    },
  }
}