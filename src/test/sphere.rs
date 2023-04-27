#![allow(unused_imports)]
use glam::Vec3;
use crate::object::Object;
use crate::object::sphere::Sphere;
use crate::ray::{Ray, RayColor};

#[test]
fn reach_point() {
    let ray = Ray::new(Vec3::ZERO, Vec3::X * 3.0, RayColor::R(255.0));
    let ray2 = Ray::new(Vec3::ZERO, Vec3::X * -1.0, RayColor::R(255.0));

    let sphere = Sphere::new(
        Vec3::new(4.0, 4.0, 0.0),
        5.0
    );

    let sphere2 = Sphere::new(
        Vec3::new(4.0, 4.0, 0.0),
        1.0
    );

    assert_eq!(Some(Vec3::new(1.0, 0.0, 0.0)), sphere.reach_point(ray));
    assert_eq!(None, sphere.reach_point(ray2));
    assert_eq!(None, sphere2.reach_point(ray));
    assert_eq!(None, sphere2.reach_point(ray2));
}

#[test]
fn reach_normal() {
    let ray = Ray::new(Vec3::ZERO, Vec3::X * 3.0, RayColor::R(255.0));
    let ray2 = Ray::new(Vec3::ZERO, Vec3::Y, RayColor::R(255.0));

    let sphere = Sphere::new(
        Vec3::new(4.0, 4.0, 0.0),
        5.0
    );

    assert_eq!(Vec3::new(-0.6, -0.8, 0.0), sphere.reach_normal(sphere.reach_point(ray).unwrap()));
    assert_eq!(Vec3::new(-0.8, -0.6, 0.0), sphere.reach_normal(sphere.reach_point(ray2).unwrap()));
}