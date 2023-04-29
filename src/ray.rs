#![allow(dead_code)]

use glam::Vec3;
use rand_distr::{Normal as Gaussian, Distribution};
use crate::material::color::Color;
use std::f32::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) source: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) color: RayColor,
    pub(crate) reached_light: bool,
    pub(crate) reflect_count: u32,
}

impl Ray {
    pub(crate) fn new(source: Vec3, direction: Vec3, color: RayColor) -> Self {
        Ray { source, direction, color, reached_light: false, reflect_count: 0 }
    }

    pub(crate) fn is_done(&self) -> bool {
        self.reflect_count >= crate::HyperParameter::REFLECTION_COUNT || self.reached_light
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum RayColor {
    R(f32),
    G(f32),
    B(f32)
}

impl RayColor {
    pub(crate) const RED: RayColor = RayColor::R(1.0);
    pub(crate) const GREEN: RayColor = RayColor::G(1.0);
    pub(crate) const BLUE: RayColor = RayColor::B(1.0);

    pub(crate) fn mix_color(&self, surface_color: Color<f32>) -> RayColor {
        match self {
            RayColor::R(r) => RayColor::R(r * surface_color.r),
            RayColor::G(g) => RayColor::G(g * surface_color.g),
            RayColor::B(b) => RayColor::B(b * surface_color.b)
        }
    }

    pub(crate) fn get_value(&self) -> f32 {
        match self {
            RayColor::R(r) => *r,
            RayColor::G(g) => *g,
            RayColor::B(b) => *b
        }
    }
}

pub(crate) trait Optics {
    fn reflect_from(self, normal: Vec3) -> Self;
    fn dispersion(self, gaussian: Gaussian<f32>) -> Self;
}

impl Optics for Vec3 {
    fn reflect_from(self, normal: Vec3) -> Self {
        self.reject_from(normal) * 2.0 - self
    }

    fn dispersion(self, rough: Gaussian<f32>) -> Self {
        const TWO_PI: f32 = 2.0 * consts::PI;
        let (unit_x, unit_y) = self.any_orthonormal_pair();

        let r_disp = rough.sample(&mut rand::thread_rng());
        let angle = rand::random::<f32>() * TWO_PI;

        let x_coor = r_disp * angle.sin() * self.length();
        let y_coor = r_disp * angle.cos() * self.length();

        self + x_coor * unit_x + y_coor * unit_y
    }
}