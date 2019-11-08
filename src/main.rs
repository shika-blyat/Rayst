pub mod vec;
pub mod ray;
use vec::Vec3;
use ray::Ray;
use std::fs;

const NX: i32= 200;
const NY: i32= 100;

fn main() {
    render();
}

fn write_in_file(data: &str){
    fs::write("image.ppm", data).expect("Unable to find file location");
}
fn render() {
	let mut data = format!("P3\n{} {}\n255\n",NX,NY);
	let lower_left = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::new(0.0, 0.0, 0.0);
	for j in (0..NY).rev(){
		for i in 0..NX{
			let u: f32 = i as f32 / NX as f32;
			let v: f32 = j as f32 / NY as f32 ;
			let r = Ray::new(origin, lower_left+ u*horizontal + v*vertical);
			let col = color(r);
			let ir = (255.99*col[0]) as i32;
			let ig = (255.99*col[1]) as i32;
			let ib = (255.99*col[2]) as i32;
			data.push_str(format!("{} {} {}\n",ir,ig,ib).as_str());
		}
	}
    write_in_file(&data);
}

fn color(r: Ray) -> Vec3 {
	let mut unit_direction = r.direction();
	unit_direction.normalize();
	let t: f32 = 0.5*(unit_direction.y() + 1.0);
	(1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}