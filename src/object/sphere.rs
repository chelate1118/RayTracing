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
        let source_to_center: Vec3 = self.center - ray.source;

        let light_dist: f32 = source_to_center
            .reject_from_normalized(ray.direction)
            .length();

        if light_dist > self.radius {
            return None;
        }

        let dot_product: f32 = Vec3::dot(source_to_center, ray.direction);
        let reach_distance: f32 = dot_product - (self.radius.powi(2) - light_dist.powi(2)).sqrt();

        if dot_product < 0.0 {
            return None;
        }

        if reach_distance < 0.0 {
            panic!("Ray can't start from inside of the sphere.");
        }

        let reach_vector: Vec3 = ray.direction * reach_distance;

        return Some(ray.source + reach_vector);
    }
}

