#[cfg(test)]
mod test_link {
  use std::{rc::Rc};

  type Link<T> = Option<Rc<Node<T>>>;

  struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
  }

  #[test]
  fn test_manual_link_with_option() {
    let mut head = Some(Rc::new(Node {
      elem: 1,
      prev: None,
      next: None,
    }));

    let next_node = Some(Rc::new(Node {
      elem: 2,
      prev: None,
      next: None,
    }));

    let mut head_unwrapped = head.unwrap();
    let mut_unwrapped = Rc::get_mut(&mut head_unwrapped).unwrap();
    mut_unwrapped.next = next_node;
  }
}
