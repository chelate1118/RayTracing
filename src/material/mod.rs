#![allow(dead_code)]

use glam::Vec3;

use crate::ray::Ray;

mod normal;
mod light_source;

pub(crate) trait Material {
    fn reflect(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray;
}