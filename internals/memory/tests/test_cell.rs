use std::cell::Cell;

#[test]
fn test_cell_simple() {
  struct SomeStruct {
    celled: Cell<u8>,
  }

  let instance = SomeStruct {
    celled: Cell::new(5),
  };

  instance.celled.set(10);
  assert_eq!(instance.celled.get(), 10);
}

#[test]
fn test_cell_structured() {
  // Copy trait is allowed for shallow copy only
  // "copying a Copy type does not run any code"
  #[derive(Copy, Clone)]
  struct Payload<'a> {
    email: &'a str,
    stamp: u32,
  }

  struct WrapperStruct<'a> {
    payload: Cell<Payload<'a>>,
  }

  let instance = WrapperStruct {
    payload: Cell::new(Payload {
      email: "test_string",
      stamp: 10,
    }),
  };

  let expected_email = "result_string";
  let expected_stamp: u32 = 5;

  instance.payload.set(Payload {
    email: expected_email,
    stamp: expected_stamp,
  });

  assert_eq!(instance.payload.get().email, expected_email);
  assert_eq!(instance.payload.get().stamp, expected_stamp);
}