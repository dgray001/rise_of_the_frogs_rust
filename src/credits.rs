pub fn credits() {
  println!("Created by Daniel Gray");
  println!("2023 03 03");
  println!("v0.0d: Help args");
}

pub fn welcome() {
  asci();
  credits();
  println!("\nWelcome to 'Rise of the Frogs'");
  println!("\nTo see the list of possible commands, type 'ls'");
  println!("For general information on how the program works, type 'help'");
}

fn asci() {
  println!(r#"  ___ _                __   _   _          ___                 "#);
  println!(r#" | _ (_)___ ___   ___ / _| | |_| |_  ___  | __| _ ___  __ _ ___"#);
  println!(r#" |   / (_-</ -_) / _ \  _| |  _| ' \/ -_) | _| '_/ _ \/ _` (_-<"#);
  println!(r#" |_|_\_/__/\___| \___/_|    \__|_||_\___| |_||_| \___/\__, /__/"#);
  println!(r#"                                                      |___/    "#);
}