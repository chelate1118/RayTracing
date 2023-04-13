use crate::ray::Ray;
use glam::Vec3;

mod sphere;

trait Object {
    fn reach_point(&self, ray: &Ray) -> Option<Vec3>;
}