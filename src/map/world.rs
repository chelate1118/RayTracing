use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{object::{Object, sphere::{normal_sphere::NormalSphere, light_sphere::{LightSphere}}, plane::Plane, sun::Sun}, ray::Ray, loader::FromValue};

pub(crate) struct World {
    pub(crate) objects: Vec<Box<dyn Object>>
}

impl FromValue for World {
    fn from_value(value: Value) -> serde_json::Result<Self> {
        let wi: WorldInfo = serde_json::from_value(value)?;
        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        objects.push(Box::new(Sun::from_value(wi.sun)?));

        for plane in wi.plane.iter() {
            objects.push(Box::new(Plane::from_value(plane.to_owned())?))
        }

        for normal_sphere in wi.normal_sphere.iter() {
            objects.push(Box::new(NormalSphere::from_value(normal_sphere.to_owned())?))
        }

        for light_sphere in wi.light_sphere.iter() {
            objects.push(Box::new(LightSphere::from_value(light_sphere.to_owned())?))
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
                if min_distance >= distance {
                    min_distance = distance;
                    reach_object = Some(object);
                }
            }
        }

        reach_object.map(Box::as_ref)
    }
}

#[derive(Serialize, Deserialize)]
struct WorldInfo {
    sun: Value,
    plane: Vec<Value>,
    normal_sphere: Vec<Value>,
    light_sphere: Vec<Value>,
}