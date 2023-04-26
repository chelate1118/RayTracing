use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{object::{Object, sphere::{normal_sphere::NormalSphere}}, ray::Ray, loader::FromValue};

pub(crate) struct World {
    pub(crate) objects: Vec<Box<dyn Object>>
}

impl FromValue for World {
    fn from_value(value: Value) -> serde_json::Result<Self> {
        let wi: WorldInfo = serde_json::from_value(value)?;
        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        for sphere in wi.normal_sphere.iter() {
            objects.push(Box::new(NormalSphere::from_value(sphere.to_owned())?))
        }

        Ok(World { objects })
    }
}

impl World {
    pub(crate) fn start_ray(&self, ray: Ray) -> Ray {
        let mut ray = ray;

        while !ray.is_done() {
            match self.reach_object(ray) {
                Some(obj) => ray = obj.reflect(ray),
                None => break
            }
        }

        ray
    }

    fn reach_object(&self, ray: Ray) -> Option<&dyn Object> {
        let mut reach_object = None;
        let mut min_distance = std::f32::MAX;
        for object in self.objects.iter() {
            if let Some(distance) = object.reach_distance(ray) {
                if min_distance > distance {
                    min_distance = distance;
                    reach_object = Some(object);
                }
            }
        }

        reach_object.map(Box::as_ref)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WorldInfo {
    plane: Vec<Value>,
    normal_sphere: Vec<Value>
}