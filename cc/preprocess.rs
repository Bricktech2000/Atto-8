use crate::*;

pub fn preprocess(input: String) -> Result<String, Error> {
  let input: String = input
    .lines()
    .map(|line| line.split("//").next().unwrap_or(line))
    .collect::<Vec<_>>()
    .join("\n");

  Ok(input)
}
