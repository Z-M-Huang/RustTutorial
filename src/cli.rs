use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  //cargo run hello
  println!("Args: {:?}", args);
}
