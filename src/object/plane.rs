#![allow(dead_code)]

use glam::Vec3;
use serde::Deserialize;
use serde_json::Value;

use crate::{material::{normal::Normal, color::Color, Material}, loader::FromValue, ray::Ray};

use super::Object;

pub(crate) struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Normal
}

impl FromValue for Plane {
    fn from_value(value: Value) -> serde_json::Result<Self> {
        let pi: PlaneInfo = serde_json::from_value(value)?;

        Ok(Plane {
            point: Vec3::from_array(pi.point),
            normal: Vec3::from_array(pi.normal).normalize(),
            material: Normal::new(
                Color::from_array(pi.color),
                pi.rough
            )
        })
    }
}

impl Object for Plane {
    fn reach_point(&self, ray: Ray) -> Option<Vec3> {
        let source_to_point: Vec3 = self.point - ray.source;
        let normalized_dir = ray.direction.normalize();

        let source_to_plane = source_to_point.project_onto(self.normal);
        let cos = source_to_plane.normalize().dot(normalized_dir);
        if cos <= 0.0 {
            return None;
        }

        let reach_distance = source_to_plane.length()/cos;

        let source_to_intersection = reach_distance * normalized_dir;

        Some(ray.source + source_to_intersection)
    }

    fn reach_normal(&self, _: Vec3) -> Vec3 {
        self.normal
    }

    fn get_material(&self, _: Vec3) -> Box<dyn Material> {
        Box::new(self.material)
    }
}

#[derive(Deserialize)]
struct PlaneInfo {
    point: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
    rough: f32
}