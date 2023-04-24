pub(crate) mod sphere;
pub(crate) mod plane;

use crate::{ray::Ray, material::Material};
use glam::Vec3;

pub(crate) trait Object {
    fn reach_point(&self, ray: Ray) -> Option<Vec3>;
    fn reach_normal(&self, ray: Ray) -> Vec3;
    fn get_material(&self) -> Box<dyn Material>; 

    fn reflect_from(&self, ray: Ray) -> Ray {
        self.get_material().reflect(
            ray,
            self.reach_normal(ray),
            self.reach_point(ray).unwrap()
        )
    }
}