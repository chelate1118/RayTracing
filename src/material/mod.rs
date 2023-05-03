#![allow(dead_code)]

use glam::Vec3;

use crate::ray::Ray;

pub(crate) mod normal;
pub(crate) mod light_source;
pub(crate) mod color;
pub(crate) mod sun;

pub(crate) trait Material {
    fn reflect(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray;
}