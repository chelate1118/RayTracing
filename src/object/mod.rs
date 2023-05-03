pub(crate) mod sphere;
pub(crate) mod plane;
pub(crate) mod sun;

use crate::{ray::Ray, material::Material};
use glam::Vec3;

pub(crate) trait Object {
    fn reach_point(&self, ray: Ray) -> Option<Vec3>;
    fn reach_normal(&self, point: Vec3) -> Vec3;
    fn get_material(&self, point: Vec3) -> Box<dyn Material>; 

    fn reach_distance(&self, ray: Ray) -> Option<f32> {
        self.reach_point(ray).map(|point|
            ray.source.distance(point)
        )
    }

    fn reflect(&self, ray: Ray) -> Ray {
        let point = self.reach_point(ray).unwrap();
        self.get_material(point).reflect(
            ray,
            self.reach_normal(point),
            point
        )
    }
}