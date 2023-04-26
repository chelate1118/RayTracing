use crate::{object::Object, ray::Ray};

pub(crate) struct World {
    pub(crate) objects: Vec<Box<dyn Object>>
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