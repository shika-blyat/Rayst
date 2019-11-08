use crate::vec::Vec3;

pub struct Ray{
	origin: Vec3,
	direction: Vec3
}

impl Ray{
	fn new(origin: Vec3, direction: Vec3) -> Ray {
		Ray {
			origin,
			direction
		}
	}
	fn origin(&self) -> Vec3 {
		self.origin
	}
	fn direction(&self) -> Vec3 {
		self.direction
	}
	fn point_at(&self, t: f32) -> Vec3 {
		self.origin + self.direction*t
	}
}
