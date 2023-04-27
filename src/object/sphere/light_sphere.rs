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
                Vec3::new(lsi.x, lsi.y, lsi.z),
                lsi.radius
            ),
            material: LightSource::new(
                Color::new(lsi.r, lsi.g, lsi.b)
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
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    r: f32,
    g: f32,
    b: f32,
    gloom: f32
}
