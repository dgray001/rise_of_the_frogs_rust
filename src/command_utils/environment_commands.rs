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
          "wait" => wait(context),
          "fight" => fight(context),
          "pickup" => pickup(context),
          "inventory" => inventory(context),
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
    if unit.despawn() || !game.player.can_view(unit) {
      unit.view_index = -1;
      continue;
    }
    unit.view_index = index;
    if index == 1 {
      output_str += "Units\n";
    }
    output_str += &format!("  {}: {}\n", index, unit.view_short(&context.unit_loader));
    index += 1;
  }
  index = 1;
  for item in game.environment.items.iter_mut() {
    if !game.player.can_view(item) {
      item.view_index = -1;
      continue;
    }
    item.view_index = index;
    if index == 1 {
      output_str += "\nItems\n";
    }
    output_str += &format!("  {}: {}\n", index, item.view_short(&context.item_loader));
    index += 1;
  }
  context.println(output_str.as_str());
}

fn wait<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  view(context);
  let game = context.curr_game.as_mut().unwrap();
  game.environment.pass_time();
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
      game.environment.pass_time();
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
  let game = context.curr_game.as_mut().unwrap();
  if !game.player.inventory.can_pickup() {
    context.println("Your inventory is out of space");
    return;
  }
  let index = context.last_params.parse::<i64>().unwrap_or(-1);
  if index < 1 {
    context.println("Invalid index. Must be a positive integer");
    return;
  }
  let mut pickup_index = None;
  for (i, item) in game.environment.items.iter().enumerate() {
    if item.view_index != index {
      continue;
    }
    pickup_index = Some(i);
  }
  match pickup_index {
    Some(i) => {
      let item = game.environment.items.remove(i);
      if !game.player.can_view(&item) {
        context.println("Item no longer in view. Use 'view' to update view");
        return;
      }
      let item_string = item.to_string();
      match game.player.inventory.add(item) {
        Some(it) => {
          game.environment.items.push(it);
          context.println("Inventory full");
        },
        None => {
          game.environment.pass_time();
          context.println(&format!("Picked up {}", item_string));
        },
      }
    },
    None => {
      context.println("Item not found")
    },
  }
}

fn inventory<R, W, E>(context: &mut context::RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  let game = context.curr_game.as_mut().unwrap();
  let mut display_string = "  -- Inventory --\n".to_owned();
  display_string += &format!("Capacity: {}\n", game.player.inventory.capacity);
  display_string += "Items:\n";
  // eventually pass param as filter to filter list
  for item in game.player.inventory.list() {
    display_string += &format!("  {}\n", item.to_string());
  }
  context.println(display_string.as_str());
}