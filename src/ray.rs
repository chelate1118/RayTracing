#![allow(dead_code)]

use glam::Vec3;
use rand_distr::{Normal as Gaussian, Distribution};
use crate::color::Color;
use std::f32::*;

#[derive(Clone, Copy)]
pub(crate) struct Ray {
    pub(crate) source: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) color: RayColor,
    pub(crate) light_source: bool,
    pub(crate) reflect_count: u32,
}

impl Ray {
    pub(crate) fn new(source: Vec3, direction: Vec3, color: RayColor) -> Self {
        return Ray { source, direction, color, light_source: false, reflect_count: 0 };
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum RayColor {
    R(f32),
    G(f32),
    B(f32)
}

impl RayColor {
    pub(crate) fn mix_color(&self, surface_color: Color) -> RayColor {
        match self {
            RayColor::R(r) => RayColor::R(r * surface_color.r),
            RayColor::G(g) => RayColor::G(g * surface_color.g),
            RayColor::B(b) => RayColor::B(b * surface_color.b)
        }
    }
}

pub(crate) trait Optics {
    fn reflect_from(self, normal: Vec3) -> Self;
    fn dispersion(self, gaussian: Gaussian<f32>) -> Self;
}

impl Optics for Vec3 {
    fn reflect_from(self, normal: Vec3) -> Self {
        return self - self.reject_from(normal) * 2.0
    }

    fn dispersion(self, rough: Gaussian<f32>) -> Self {
        const TWO_PI: f32 = 2.0 * consts::PI;
        let (unit_x, unit_y) = self.any_orthonormal_pair();

        let r_disp = rough.sample(&mut rand::thread_rng());
        let angle = rand::random::<f32>() * TWO_PI;

        let x_coor = r_disp * angle.sin() * self.abs();
        let y_coor = r_disp * angle.cos() * self.abs();

        return self + x_coor * unit_x + y_coor * unit_y;
    }
}