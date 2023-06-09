use glam::Vec3;
use rand_distr::Normal as Gaussian;
use crate::{ray::{Ray, Optics}, material::color::Color};
use super::Material;

#[derive(Clone, Copy)]
pub(crate) struct Normal {
    rough: Gaussian<f32>,
    color: Color<f32>
}

impl Normal {
    pub(crate) fn new(color: Color<f32>, rough: f32) -> Self {
        if color.r > 255.0 || color.g > 255.0 || color.b > 255.0 {
            panic!("Normal surface color value must be less than 255.0.");
        }

        let color = color / 255.0;

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
        let color = ray.color * self.color;
        let light_source = ray.reached_light;
        let reflect_count = ray.reflect_count + 1;

        direction = direction.dispersion(self.rough);
        while is_invalid_reflect(ray.direction, direction, normal) {
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

fn is_invalid_reflect(before: Vec3, after: Vec3, normal: Vec3) -> bool {
    before.dot(normal) * after.dot(normal) > 0.0
}