// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  // Primitive
  let hello = "Hello ";
  let mut hello_mut = String::from("Hello ");

  // Get length
  println!("Length: {}", hello.len());

  hello_mut.push('W');

  hello_mut.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello_mut.capacity());

  // Check empty
  println!("Is Empty: {}", hello_mut.is_empty());

  // Contains
  println!("Contains 'World' {}", hello_mut.contains("World"));

  // Replace
  println!("Replace: {}", hello_mut.replace("World", "There"));

  // Loop through string by whitespace
  for word in hello_mut.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{:?}", (hello, hello_mut, s));
}
