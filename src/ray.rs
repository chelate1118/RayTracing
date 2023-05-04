use glam::{Vec3, Quat};
use rand_distr::{Normal as Gaussian, Distribution};
use crate::material::color::Color;
use std::f32::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) source: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) color: Color<f32>,
    pub(crate) reached_light: bool,
    pub(crate) reflect_count: u32,
}

impl Ray {
    pub(crate) fn new(source: Vec3, direction: Vec3, color: Color<f32>) -> Self {
        Ray { source, direction, color, reached_light: false, reflect_count: 0 }
    }

    pub(crate) fn is_done(&self, reflect_count: u32) -> bool {
        self.reflect_count >= reflect_count || self.reached_light
    }
}

pub(crate) trait Optics {
    const TWO_PI: f32 = 2.0 * consts::PI;
    fn reflect_from(self, normal: Vec3) -> Self;
    fn dispersion(self, gaussian: Gaussian<f32>) -> Self;
    fn random_orthonormal(self) -> Self;
}

impl Optics for Vec3 {
    fn reflect_from(self, normal: Vec3) -> Self {
        self.reject_from(normal) * 2.0 - self
    }

    fn dispersion(self, rough: Gaussian<f32>) -> Self {        
        let rotate_axis = self.random_orthonormal();
        let angle = rough.sample(&mut rand::thread_rng());
        let quat = Quat::from_axis_angle(rotate_axis, angle);

        quat.mul_vec3(self)
    }

    fn random_orthonormal(self) -> Self {
        let (unit_x, unit_y) = self.any_orthonormal_pair();
        let angle = rand::random::<f32>() * Self::TWO_PI;

        unit_x * angle.sin() + unit_y * angle.cos()
    }
}