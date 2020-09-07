pub fn run() {
  println!("Hello from the print.rs file");

  println!("{} is from {}", "Brad", "Mass");

  //Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Brad", "Mass", "code"
  );

  //Named Arguments
  println!(
    "{name} likes to play {acitivity}",
    name = "John",
    acitivity = "Baseball"
  );

  //Traits
  println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

  //Debug Traits. Tuple
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
