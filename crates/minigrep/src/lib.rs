use std::{env, error::Error, fs, path};

pub struct Config {
  pub query: String,
  pub file_path: String,
  ignore_case: bool,
}

impl Config {
  pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("[x] Failed to get query"),
    };
    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err("[x] Failed to get file_path"),
    };
    println!("[+] Search `{}` in `{}`", query, file_path);
    if !path::Path::new(&file_path).exists() {
      return Err("[x] Not found file");
    }

    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Ok(Config { query, file_path, ignore_case })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.file_path)?;

  let result = if config.ignore_case {
    search_insensitive(&config.query, &content)
  } else {
    search(&config.query, &content)
  };
  for line in result {
    println!("[+] search line: {}", line);
  }

  Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content.lines().filter(|line| line.contains(query)).collect()
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content.lines().filter(|line| line.to_lowercase().contains(query)).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search() {
    let query = "not";
    let content = "\
Oh, God, give us courage to change what must be altered,
serenity to accept what can not be helped,
and insight to know the one from the other.";
    assert_eq!(vec!["serenity to accept what can not be helped,"], search(query, content))
  }

  #[test]
  fn test_case_insensitive() {
    let query = "god";
    let content = "\
Oh, God, give us courage to change what must be altered,
serenity to accept what can not be helped,
and insight to know the one from the other.";
    assert_eq!(
      vec!["Oh, God, give us courage to change what must be altered,"],
      search_insensitive(query, content)
    )
  }
}
