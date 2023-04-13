mod object;
mod ray;

use glam::Vec3;

pub struct HyperParameter {}
impl HyperParameter {
    #[allow(dead_code)]
    const REFLECTION_COUNT: i32 = 2;
}

fn main() {
    let v31 = Vec3::X;
    let v32 = Vec3::new(2.0, 3.0, 1.0);

    println!("{}", Vec3::normalize(v32));
    println!("{}", v32);
}
