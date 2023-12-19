fn eat_box_i32(boxed_i32: Box<i32>) {
  println!("moved value {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
  println!("int is {}", borrowed_i32);
}

#[test]
fn initial() {
  let boxed_i32 = Box::new(5_i32);
  let stacked_i32 = 6_i32;

  borrow_i32(&boxed_i32);
  borrow_i32(&stacked_i32);

  eat_box_i32(boxed_i32);

  // borrow_i32(&boxed_i32); //compile error because box value moved
}