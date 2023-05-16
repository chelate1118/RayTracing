use serde::Deserialize;
use serde_json::Value;

use crate::{object::{Object, sphere::{normal_sphere::NormalSphere, light_sphere::LightSphere, glass_sphere::GlassSpere}, plane::Plane, sun::Sun}, ray::Ray, loader::FromValue};

pub(crate) struct World {
    pub(crate) objects: Vec<Box<dyn Object>>
}

impl FromValue for World {
    fn from_value(value: Value) -> serde_json::Result<Self> {
        let wi: WorldInfo = serde_json::from_value(value)?;
        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        if let Some(sun) = wi.sun {
            objects.push(Box::new(Sun::from_value(sun)?));
        }

        if let Some(planes) = wi.plane {
            for plane in planes.iter() {
                objects.push(Box::new(Plane::from_value(plane.to_owned())?))
            }
        }

        if let Some(normal_spheres) = wi.normal_sphere {
            for normal_sphere in normal_spheres.iter() {
                objects.push(Box::new(NormalSphere::from_value(normal_sphere.to_owned())?))
            }
        }

        if let Some(light_spheres) = wi.light_sphere {
            for light_sphere in light_spheres.iter() {
                objects.push(Box::new(LightSphere::from_value(light_sphere.to_owned())?))
            }
        }

        if let Some(glass_spheres) = wi.glass_sphere {
            for glass_sphere in glass_spheres.iter() {
                objects.push(Box::new(GlassSpere::from_value(glass_sphere.to_owned())?))
            }
        }

        Ok(World { objects })
    }
}

impl World {
    pub(crate) fn start_ray(&self, ray: Ray, reflect_count: u32) -> Ray {
        let mut ray = ray;

        while !ray.is_done(reflect_count) {
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

#[derive(Deserialize)]
struct WorldInfo {
    sun: Option<Value>,
    plane: Option<Vec<Value>>,
    normal_sphere: Option<Vec<Value>>,
    light_sphere: Option<Vec<Value>>,
    glass_sphere: Option<Vec<Value>>
}