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
fn test_as_ptr_for_struct() {
  let first = Some(
    NonNull::new(&mut Node {
      elem: 1,
      prev: None,
      next: None,
    }).expect("empty")
  );

  let second = Some(
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

  let end_of_chain = unsafe { first.unwrap().as_ref().next.unwrap().as_ref() };
  assert_eq!(end_of_chain.elem, 2);
}

#[test]
fn test_as_mut_for_struct() {
  let first = Some(
    NonNull::new(&mut Node {
      elem: 1,
      prev: None,
      next: None,
    }).expect("empty")
  );

  let second = Some(
    NonNull::new(&mut Node {
      elem: 2,
      prev: None,
      next: None,
    }).expect("empty")
  );

  unsafe { first.unwrap().as_mut().next = second };

  let end_of_chain = unsafe { first.unwrap().as_ref().next.unwrap().as_ref() };
  assert_eq!(end_of_chain.elem, 2);
}