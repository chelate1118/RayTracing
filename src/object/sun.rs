use glam::Vec3;
use serde::Deserialize;
use serde_json::{Value, Result};

use crate::{material::color::Color, loader::FromValue};

use super::Object;

#[derive(Clone, Copy)]
pub(crate) struct Sun {
    pub(crate) direction: Vec3,
    pub(crate) color: Color<f32>,
    pub(crate) dispertion: f32
}

impl FromValue for Sun {
    fn from_value(value: Value) -> Result<Self> {
        let si: SunInfo = serde_json::from_value(value)?;
        
        Ok(Sun {
            direction: Vec3::from_array(si.direction).normalize(),
            color: Color::from_array(si.color),
            dispertion: si.dispertion
        })
    }
}

impl Object for Sun {
    fn reach_point(&self, _: crate::ray::Ray) -> Option<Vec3> {
        unreachable!()
    }

    fn reach_normal(&self, _: Vec3) -> Vec3 {
        unreachable!()
    }

    fn get_material(&self, _: Vec3) -> Box<dyn crate::material::Material> {
        Box::new(self.to_owned())
    }

    fn reach_distance(&self, _: crate::ray::Ray) -> Option<f32> {
        Some(std::f32::MAX)
    }

    fn reflect(&self, ray: crate::ray::Ray) -> crate::ray::Ray {
        self.get_material(Vec3::ZERO).reflect(ray, Vec3::ZERO, Vec3::ZERO)
    }
}

#[derive(Deserialize)]
struct SunInfo {
    direction: [f32; 3],
    color: [f32; 3],
    dispertion: f32
}