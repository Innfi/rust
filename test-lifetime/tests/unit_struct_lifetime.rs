use core::fmt;

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

impl<'a> fmt::Display for Person<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "name: {}, age: {}", self.name, self.age)
  }
}

#[test]
fn derive_debug() {
  let name ="peter";
  let age = 27;
  let peter = Person { name, age };

  println!("{:#}?", peter);
}