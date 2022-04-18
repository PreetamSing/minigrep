use std::{env, process};

use minigrep::Config;

fn main() {
  // let args: Vec<String> = env::args().collect();

  let config = Config::new(&mut env::args()).unwrap_or_else(|err| {
    eprintln!("Problems parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(config) {
    eprintln!("{}", err);
    process::exit(2);
  }
}
