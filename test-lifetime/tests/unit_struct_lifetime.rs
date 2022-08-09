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

impl<'a> Person<'a> {
  fn new(name: &'a str) -> Person {
    Self { 
      name,
      age: name.len() as u8,
    }
  }
}

#[test]
fn derive_debug() {
  let name ="peter";
  let age = 27;
  let peter = Person { name, age };

  println!("{:#}?", peter);
}

#[test]
fn struct_initialize_order() {
  let peter = Person::new("pater");

  assert_eq!(peter.age, 5);
}