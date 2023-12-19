#[cfg(test)]
mod test_arc {
  use std::sync::Arc;
  use std::thread;

  #[test]
  fn test_arc_thread() {
    let mut five = Arc::new(5);

    for _ in 0..10{
      let five = Arc::clone(&five);

      thread::spawn(move || {
        println!("{five:?}");
      });
    }

    five = Arc::new(10);

    let mut unwrapped = Arc::try_unwrap(five).unwrap();
    unwrapped -= 1;

    assert_eq!(unwrapped, 9);
  }
}