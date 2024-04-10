use minigrep::Config;
use std::{env, process};

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("{}", err);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(config) {
    eprintln!("{}", err);
    process::exit(1);
  }
}
