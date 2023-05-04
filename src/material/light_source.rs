use std::ops::Mul;

use glam::Vec3;

use crate::{ray::Ray, material::color::Color};
use super::Material;

#[derive(Clone, Copy)]
pub(crate) struct LightSource {
    color: Color<f32>
}

impl Mul<f32> for LightSource {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        LightSource { color: self.color * rhs }
    }
}

impl LightSource {
    pub(crate) fn new(color: Color<f32>) -> Self {
        if color.r < 1.0 && color.g < 1.0 && color.b < 1.0 {
            panic!("Light Source should be bright");
        }

        LightSource { color }
    }
}

impl Material for LightSource {
    fn reflect(&self, ray: Ray, _: Vec3, point: Vec3) -> Ray {
        Ray {
            source: point,
            direction: ray.direction,
            color: ray.color.mix_color(self.color),
            reached_light: true,
            reflect_count: 0
        }
    }
}