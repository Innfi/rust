use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn test_refcell2() {
  // "RefCell<T>s are for single-threaded scenarios"
  let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

  {
    let mut map: RefMut<'_, _> = shared_map.borrow_mut();
    map.insert("test1", 12);
    map.insert("test2", 14);
    map.insert("test3", 22);
  }

  let total: i32 = shared_map.borrow().values().sum();
  println!("{total}");

  assert_eq!(total, 48);
}