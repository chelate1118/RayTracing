#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod loader;
mod map;
mod test;

use glam::Vec3;

pub struct HyperParameter {}
impl HyperParameter {
    const REFLECTION_COUNT: u32 = 2;
}

fn main() {
    let world = map::world::World { objects: Vec::new() };
    world.start_ray(ray::Ray::new(
        Vec3::ZERO,
        Vec3::X,
        ray::RayColor::RED
    ));
}
