// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
  #[derive(Copy, Clone)] // You can get compiler to implement things for you :D
  struct STest {
    key: i32,
    key2: i32,
  }

  let test = STest { key: 1, key2: 2 };

  let a = [test; 100];

  if a.len() >= 100 {
    println!("Wow, that's a big array!");
  } else {
    println!("Meh, I eat arrays like that for breakfast.");
  }
}
