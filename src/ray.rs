use crate::vec::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray{
	origin: Vec3,
	direction: Vec3
}

impl Ray{
	pub fn new(origin: Vec3, direction: Vec3) -> Ray {
		Ray {
			origin,
			direction
		}
	}
	pub fn origin(&self) -> Vec3 {
		self.origin
	}
	pub fn direction(&self) -> Vec3 {
		self.direction
	}
	pub fn point_at(&self, t: f32) -> Vec3 {
		self.origin + self.direction*t
	}
}
