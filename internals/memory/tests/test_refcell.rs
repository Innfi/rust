use std::cell::{RefCell, RefMut};

#[test]
fn test_refcell() {
  let c = RefCell::new((5, 'b'));
  {
    let b1: RefMut<(u32, char)> = c.borrow_mut();
    let mut b2: RefMut<u32> = RefMut::map(b1, |t| &mut t.0);

    assert_eq!(*b2, 5);
    *b2 = 42;
  }

  assert_eq!(*c.borrow(), (42, 'b'));
}
