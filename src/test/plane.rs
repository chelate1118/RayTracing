#![allow(unused_imports)]

use glam::Vec3;

use crate::{ray::{Ray, RayColor}, object::{plane::Plane, Object}, loader::{FromValue, str_to_value}};

#[test]
fn reach_point() {
    use super::assert_eq_vec3;
    
    let ray = Ray::new(
        Vec3::Z, -Vec3::Z, RayColor::RED
    );

    let ray2 = Ray::new(
        Vec3::Z, Vec3::Z, RayColor::RED
    );

    let plane = Plane::from_value(
        str_to_value(r#"
            {
                "px": 1,
                "py": 0,
                "pz": 0,
                "nx": -1,
                "ny": 0,
                "nz": 1,
                "r": 255,
                "g": 255,
                "b": 255,
                "rough": 3
            }
        "#).unwrap()
    ).unwrap();

    assert_eq_vec3!(
        plane.reach_point(ray).unwrap(),
        -Vec3::Z
    );

    assert_eq!(
        plane.reach_point(ray2),
        None
    )
}