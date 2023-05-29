use std::io;
use std::io::Write;

pub fn get_input(msg: &str) -> String {
  print!("{}", msg);

  io::stdout().flush().unwrap();


  let mut input: String = String::new();
  io::stdin().read_line(&mut input).expect("Reading error");

  input.trim().to_string()
}