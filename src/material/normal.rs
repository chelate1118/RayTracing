use glam::Vec3;

use crate::ray::Ray;

use super::Material;

#[derive(Clone, Copy)]
pub(crate) struct Normal {

}

impl Material for Normal {
    fn reflect(&self, ray: Ray, normal: Vec3) -> Ray {
        
        todo!()
    }
}