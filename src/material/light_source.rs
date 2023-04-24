use glam::Vec3;

use crate::{ray::Ray, color::Color};
use super::Material;

#[derive(Clone, Copy)]
pub(crate) struct LightSource {
    color: Color
}

impl LightSource {
    pub(crate) fn new(color: Color) -> Self {
        if color.r < 1.0 && color.g < 1.0 && color.b < 1.0 {
            panic!("Light Source should be bright");
        }

        LightSource { color }
    }
}

impl Material for LightSource {
    fn reflect(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray {
        Ray {
            source: point,
            direction: ray.direction,
            color: ray.color.mix_color(self.color),
            light_source: true,
            reflect_count: 0
        }
    }
}