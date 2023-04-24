pub(crate) mod sphere;
pub(crate) mod plane;

use crate::ray::Ray;
use glam::Vec3;

pub(crate) trait Object {
    fn reach_point(&self, ray: Ray) -> Option<Vec3>;
}