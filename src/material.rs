#![allow(dead_code)]

use glam::Vec3;

use crate::ray::Ray;

mod normal;
mod lightsource;

pub(crate) trait Material {
    fn reflect(&self, ray: Ray, normal: Vec3) -> Ray;
}