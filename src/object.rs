mod sphere;
mod plane;

use crate::ray::Ray;
use glam::Vec3;

trait Object {
    fn reach_point(&self, ray: Ray) -> Option<Vec3>;
}