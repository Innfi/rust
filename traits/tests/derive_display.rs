use std::fmt;

struct Point2d {
	x: f64,
	y: f64,
}

impl fmt::Display for Point2d {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Point2d] x: {}, y: {}", self.x, self.y)
	}
}

#[test]
fn test_display_impl() {
	let instance = Point2d { x: 1.5, y: 2.1 };

	assert_eq!(format!("{}", instance), "Point2d] x: 1.5, y: 2.1");
}