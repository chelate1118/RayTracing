#![allow(dead_code)]

use glam::Vec3;
use crate::ray::Ray;
use super::Object;

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        return Sphere { center, radius };
    }
}

impl Object for Sphere {
    fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        let source_to_center: Vec3 = self.center - ray.get_source();

        let light_dist: f32 = source_to_center
            .reject_from_normalized(ray.get_direction())
            .length();

        if light_dist > self.radius {
            return None;
        }

        let dot_product: f32 = Vec3::dot(source_to_center, ray.get_direction());
        let reach_distance: f32 = dot_product - (self.radius.powi(2) - light_dist.powi(2)).sqrt();

        if dot_product < 0.0 {
            return None;
        }

        if reach_distance < 0.0 {
            panic!("Ray can't start from inside of the sphere.");
        }

        let reach_vector: Vec3 = ray.get_direction() * reach_distance;

        return Some(ray.get_source() + reach_vector);
    }
}

mod test_sphere {
    #![allow(unused_imports)]
    use glam::Vec3;
    use crate::object::Object;
    use crate::object::sphere::Sphere;
    use crate::ray::Ray;

    #[test]
    fn reach_point() {
        let ray = Ray::new(Vec3::ZERO, Vec3::X * 3.0, 10.0);
        let ray2 = Ray::new(Vec3::ZERO, Vec3::X * -1.0, 10.0);

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
}