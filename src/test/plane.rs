#![allow(unused_imports)]

use glam::Vec3;

use crate::{ray::Ray, object::{plane::Plane, Object}, loader::{FromValue, str_to_value}, material::color::Color};

#[test]
fn reach_point() {
    use super::assert_eq_vec3;
    
    let ray = Ray::new(
        Vec3::Z, -Vec3::Z, Color::<f32>::default()
    );

    let ray2 = Ray::new(
        Vec3::Z, Vec3::Z, Color::<f32>::default()
    );

    let plane = Plane::from_value(
        str_to_value(r#"
            {
                "point": [1, 0, 0],
                "normal": [-1, 0, 1],
                "color": [255, 255, 255],
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