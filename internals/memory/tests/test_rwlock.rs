use std::sync::RwLock;

#[test]
fn test_rwlock() {
  // RwLock: multiple reader, single writer on multi thread env
  let lock = RwLock::new(5);

  {
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();

    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
  }

  {
    // might need try_write() for practical use cases
    let mut w = lock.write().unwrap();
    *w += 1;

    assert_eq!(*w, 6);

    // scope prevents write lock to live on
  }

  let mut another_write = lock.write().unwrap();
  *another_write += 1;
  assert_eq!(*another_write, 7);
}