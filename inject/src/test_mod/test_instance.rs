use ::inject::*;

pub struct Connection(isize);

impl Connection {
  #[inject]
  fn new(foo: isize) -> Self {
    Self(foo)
  }
}

pub struct Instance(isize);

impl Instance {
  #[inject]
  fn new(conn: &Connection) -> Self {
    Self(conn.0)
  }

	pub fn tester() {
		let conn = Box::new(Connection(2));
		let container = container![ ref conn ];
	
		let _instance = get!(&container, Instance).unwrap();
	
	}
}
