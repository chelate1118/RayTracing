#![allow(dead_code)]

use glam::Vec3;
use crate::material::color::Color;
use crate::material::Material;
use crate::material::normal::Normal;
use crate::ray::Ray;
use super::Object;

#[derive(Clone, Copy)]
pub(crate) struct NormalSphere {
    sphere: Sphere,
    material: Normal
}

impl NormalSphere {
    pub(crate) fn new(center: Vec3, radius: f32, color: Color, rough: f32) -> Self {
        NormalSphere {
            sphere: Sphere::new(center, radius),
            material: Normal::new(color, rough)
        }
    }
}

impl Object for NormalSphere {
    fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        self.sphere.reach_point(ray)
    }

    fn reach_normal(&self, ray: Ray) -> Vec3 {
        self.sphere.reach_normal(ray)
    }

    fn get_material(&self) -> Box<dyn Material> {
        Box::new(self.material)
    }    
}

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

        return Some(ray.source + reach_vector);
    }

    pub(crate) fn reach_normal(&self, ray: Ray) -> Vec3 {
        let reach_point = self.reach_point(ray).unwrap();

        (reach_point - self.center).normalize()
    }
}
