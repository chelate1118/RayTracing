use glam::Vec3;
use serde::Deserialize;

use crate::{material::{glass::Glass, color::Color}, loader::FromValue, object::Object};

use super::Sphere;

pub(crate) struct GlassSpere {
    sphere: Sphere,
    material: Glass
}

impl FromValue for GlassSpere {
    fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
        let gsi: GlassSphereInfo = serde_json::from_value(value)?;

        Ok(GlassSpere {
            sphere: Sphere::new(
                Vec3::from_array(gsi.center),
                gsi.radius
            ),
            material: Glass::new(
                Color::from_array(gsi.color),
                gsi.rough,
                gsi.reflect,
                gsi.refractive
            )
        })
    }
}

impl Object for GlassSpere {
    fn reach_point(&self, ray: crate::ray::Ray) -> Option<Vec3> {
        self.sphere.reach_point(ray)
    }

    fn reach_normal(&self, point: Vec3) -> Vec3 {
        self.sphere.reach_normal(point)
    }

    fn get_material(&self, _: Vec3) -> Box<dyn crate::material::Material> {
        Box::new(self.material)
    }
}

#[derive(Deserialize)]
struct GlassSphereInfo {
    center: [f32; 3],
    radius: f32,
    color: [f32; 3],
    rough: f32,
    reflect: f64,
    refractive: f32
}