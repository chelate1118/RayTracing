#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod world;
mod test;

use glam::Vec3;

pub struct HyperParameter {}
impl HyperParameter {
    const REFLECTION_COUNT: u32 = 2;
}

fn main() {
    let v31 = Vec3::X; 
    let v32 = Vec3::new(2.0, 3.0, 1.0);

    println!("{}", Vec3::normalize(v32));
    println!("{}", v32);
}
