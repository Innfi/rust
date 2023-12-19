fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("x: {}, y: {}", x, y);
}

// fn failed_borrow<'a>() {
//   let _x = 12;

//   let y: &'a i32 = &_x;
// }

fn valid_borrow<'a>(input: &'a mut i32) {
  let x: i32 = 12;

  *input = x;
}

#[test]
fn borrow_with_lifetime() {
  let (four, nine) = (4, 9);
  print_refs(&four, &nine);

  let mut source: i32 = 123;
  valid_borrow(&mut source);
  assert_eq!(source, 12);
}