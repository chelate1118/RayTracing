use glam::Vec3;
use noise::{Perlin, NoiseFn};
use serde::{Serialize, Deserialize};

use crate::{material::{light_source::LightSource, color::Color, Material}, loader::FromValue, object::Object, ray::Ray};

use super::Sphere;

pub(crate) struct LightSphere {
    sphere: Sphere,
    material: LightSource,
    noise: Perlin,
    gloom: f32
}

impl FromValue for LightSphere {
    fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
        let lsi: LightSphereInfo = serde_json::from_value(value)?;

        Ok(LightSphere {
            sphere: Sphere::new(
                Vec3::from_array(lsi.center),
                lsi.radius
            ),
            material: LightSource::new(
                Color::from_array(lsi.color)
            ),
            noise: Perlin::new(1),
            gloom: lsi.gloom
        })
    }
}

impl LightSphere {
    fn get_bright(&self, point: Vec3) -> f32 {
        let noise_value = self.noise.get(point.as_dvec3().to_array()) as f32;

        1.0 - self.gloom * noise_value
    }
}

impl Object for LightSphere {
    fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        self.sphere.reach_point(ray)
    }

    fn reach_normal(&self, point: Vec3) -> Vec3 {
        self.sphere.reach_normal(point)
    }

    fn get_material(&self, point: Vec3) -> Box<dyn Material> {
        Box::new(self.material * self.get_bright(point))
    }
}

#[derive(Serialize, Deserialize)]
struct LightSphereInfo {
    center: [f32; 3],
    radius: f32,
    color: [f32; 3],
    gloom: f32
}
