#![allow(dead_code)]

pub(crate) mod normal_sphere;

use glam::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub(crate) struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub(crate) fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub(crate) fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        let source_to_center: Vec3 = self.center - ray.source;
        let normalized_dir = ray.direction.normalize();

        let light_dist: f32 = source_to_center
            .reject_from_normalized(normalized_dir)
            .length();

        if light_dist > self.radius {
            return None;
        }

        let dot_product: f32 = Vec3::dot(source_to_center, normalized_dir);
        let reach_distance: f32 = dot_product - (self.radius.powi(2) - light_dist.powi(2)).sqrt();

        if dot_product < 0.0 {
            return None;
        }

        if reach_distance < 0.0 {
            panic!("Ray can't start from inside of the sphere.");
        }

        let reach_vector: Vec3 = normalized_dir * reach_distance;

        Some(ray.source + reach_vector)
    }

    pub(crate) fn reach_normal(&self, ray: Ray) -> Vec3 {
        let reach_point = self.reach_point(ray).unwrap();

        (reach_point - self.center).normalize()
    }
}
