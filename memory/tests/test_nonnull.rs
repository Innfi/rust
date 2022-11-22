use std::ptr::NonNull;

#[test]
fn test_as_mut() {
  let mut x = 0u32;
  let mut ptr = NonNull::new(&mut x).expect("null pointer");

  let x_ref = unsafe { ptr.as_mut() };
  assert_eq!(*x_ref, 0);

  *x_ref += 2;
  assert_eq!(*x_ref, 2);
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
  elem: T,
  prev: Link<T>,
  next: Link<T>,
}

#[test]
fn test_as_mut_for_struct() {
  let mut first = Some(
    NonNull::new(&mut Node {
      elem: 1,
      prev: None,
      next: None,
    }).expect("empty")
  );

  let mut second = Some(
    NonNull::new(&mut Node {
      elem: 2,
      prev: None,
      next: None,
    }).expect("empty")
  );

  let first_ref =  first.unwrap().as_ptr();
  unsafe { (*first_ref).next = second };

  let second_ref = second.unwrap().as_ptr();
  unsafe { (*second_ref).prev = first };
}