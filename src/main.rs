pub mod vec;
pub mod ray;
use vec::Vec3;
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
	for j in (0..NY).rev(){
		for i in 0..NX{
			let col = Vec3::new(i as f32/ NX as f32, j as f32/ NY as f32, 0.2);
			let ir = (255.99*col[0]) as i32;
			let ig = (255.99*col[1]) as i32;
			let ib = (255.99*col[2]) as i32;
			data.push_str(format!("{} {} {}\n",ir,ig,ib).as_str());
		}
	}
    write_in_file(&data);
}
