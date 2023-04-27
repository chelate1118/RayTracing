use glam::Vec3;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{material::{normal::Normal, color::Color, Material}, object::Object, ray::Ray, loader::FromValue};

use super::Sphere;

pub(crate) struct NormalSphere {
    sphere: Sphere,
    material: Normal
}

impl FromValue for NormalSphere {
    fn from_value(value: Value) -> serde_json::Result<Self> {
        let nsi: NormalSphereInfo = serde_json::from_value(value)?;

        Ok(NormalSphere {
            sphere: Sphere::new(
                Vec3::new(nsi.x, nsi.y, nsi.z),
                nsi.radius
            ),
            material: Normal::new(
                Color::new(nsi.r, nsi.g, nsi.b),
                nsi.rough
            )
        })
    }
}

impl Object for NormalSphere {
    fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        self.sphere.reach_point(ray)
    }

    fn reach_normal(&self, point: Vec3) -> Vec3 {
        self.sphere.reach_normal(point)
    }

    fn get_material(&self, _: Vec3) -> Box<dyn Material> {
        Box::new(self.material)
    }    
}

#[derive(Serialize, Deserialize)]
struct NormalSphereInfo {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    r: f32,
    g: f32,
    b: f32,
    rough: f32
}