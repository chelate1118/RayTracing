#![allow(unused_imports)]

use glam::Vec3;
use crate::{ray::{Optics, RayColor, Ray}, material::color::Color};


#[test]
fn reflect_color() {
    let ray_color_r = RayColor::R(40.0);
    let ray_color_g = RayColor::G(40.0);
    let ray_color_b = RayColor::B(40.0);

    let surface_color = Color::new(0.25, 0.5, 1.0);

    assert_eq!(ray_color_r.mix_color(surface_color), RayColor::R(10.0));
    assert_eq!(ray_color_g.mix_color(surface_color), RayColor::G(20.0));
    assert_eq!(ray_color_b.mix_color(surface_color), RayColor::B(40.0));
}

#[test]
fn reflect_direction() {
    let normal = Vec3::Z;
    let ray_direction = Vec3::new(1.0, 1.0, 2.0);

    assert_eq!(ray_direction.reflect_from(normal), Vec3::new(1.0, 1.0, -2.0));
}

#[test]
fn reflect_from() {
    use super::assert_eq_vec3;

    let normal = Vec3::Z;
    let dist = Vec3::X;

    assert_eq_vec3!(dist.reflect_from(normal), Vec3::X);
}