use std::str::FromStr;
use glam::Vec3;
use rand_distr::Normal as Gaussian;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::{ray::{Ray, Optics}, material::color::Color};
use super::Material;

#[derive(Clone, Copy)]
pub(crate) struct Normal {
    rough: Gaussian<f32>,
    color: Color
}

impl Normal {
    fn from_value(value: Value) -> Result<Self, serde_json::Error> {
        let ni: NormalInfo = serde_json::from_value(value)?;

        Ok(Self::new(ni.color, ni.rough))
    }

    pub(crate) fn new(color: Color, rough: f32) -> Self {
        if color.r > 1.0 || color.g > 1.0 || color.b > 1.0 {
            panic!("Normal surface color value must be less than 1.0.");
        }

        Normal {
            rough: Gaussian::new(0.0, rough).unwrap(),
            color
        }
    }
}

impl Material for Normal {
    fn reflect(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray {
        let source = point;
        let mut direction = ray.direction.reflect_from(normal);
        let color = ray.color.mix_color(self.color);
        let light_source = ray.reached_light;
        let reflect_count = ray.reflect_count + 1;
        
        direction = direction.dispersion(self.rough);
        while is_valid_reflect(ray.direction, direction, normal) {
            direction = direction.dispersion(self.rough);
        }

        Ray {
            source,
            direction,
            color,
            reached_light: light_source,
            reflect_count
        }
    }
}

fn is_valid_reflect(before: Vec3, after: Vec3, normal: Vec3) -> bool {
    before.dot(normal) * after.dot(normal) < 0.0
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NormalInfo {
    rough: f32,
    color: Color
}