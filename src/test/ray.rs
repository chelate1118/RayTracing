#![allow(unused_imports)]

use glam::Vec3;
use crate::{ray::{Optics, Ray}, material::color::Color};


#[test]
fn reflect_color() {
    let ray_color = Color::<f32>::default();

    let surface_color: Color<f32> = Color { r: 0.25, g: 0.5, b: 1.0 };

    assert_eq!(
        ray_color * surface_color,
        Color {
            r: 0.25f32,
            g: 0.5f32,
            b: 1.0f32
        }
    );
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