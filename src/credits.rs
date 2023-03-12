use std::io::{Write, BufRead};

use crate::context::RotfContext;

pub fn credits<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println("Created by Daniel Gray");
  context.println("2023 03 11");
  context.println("v0.1: Initial Infrastructure");
}

pub fn welcome<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  asci(context);
  credits(context);
  context.println("\nWelcome to 'Rise of the Frogs'");
  context.println("\nTo see the list of possible commands, type 'ls'");
  context.println("For general information on how the program works, type 'help'");
}

fn asci<R, W, E>(context: &mut RotfContext<R, W, E>) where
  R: BufRead,
  W: Write,
  E: Write,
{
  context.println(r#"  ___ _                __   _   _          ___                 "#);
  context.println(r#" | _ (_)___ ___   ___ / _| | |_| |_  ___  | __| _ ___  __ _ ___"#);
  context.println(r#" |   / (_-</ -_) / _ \  _| |  _| ' \/ -_) | _| '_/ _ \/ _` (_-<"#);
  context.println(r#" |_|_\_/__/\___| \___/_|    \__|_||_\___| |_||_| \___/\__, /__/"#);
  context.println(r#"                                                      |___/    "#);
}