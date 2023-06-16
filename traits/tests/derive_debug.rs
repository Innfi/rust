use std::fmt;

#[derive(Debug)]
enum UserType {
	TypeA,
	TypeB,
	TypeC,
}

#[derive(Debug)]
struct User {
	name: String,
	id: u32,
	email: String,
	user_type: UserType,
}

struct Group {
	name: String,
	id: u32,
	members: Vec<User>,
}

impl fmt::Debug for Group {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Group")
			.field("name", &self.name)
			.field("id", &self.name)
			.field("members", &self.members)
			.finish()
	}
}

#[test]
fn test_derive_debug_macro() {
	let instance = User {
		name: String::from("innfi"),
		id: 1,
		email: String::from("test@innfi.com"),
		user_type: UserType::TypeA,
	};

	assert_eq!(format!("{:?}", instance), 
	"User { name: \"innfi\", id: 1, email: \"test@innfi.com\", user_type: TypeA }");
}

#[test]
fn test_derive_debug_impl() {
	let instance = Group {
		name: String::from("innfisgroup"),
		id: 44,
		members: vec![],
	};

	assert_eq!(format!("{:?}", instance),
	"Group { name: \"innfisgroup\", id: \"innfisgroup\", members: [] }");
}