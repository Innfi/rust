
struct TestStruct {
  data: String,
}

impl Drop for TestStruct {
  fn drop(&mut self) {
    println!("TestStruct.drop called");
  }
}

#[test]
fn test_drop() {
  let instance = TestStruct {
    data: String::from("test")
  };

  assert_eq!(instance.data.as_str(), "test");

  drop(instance);
}