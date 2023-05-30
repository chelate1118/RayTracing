use std::f32::consts::PI;

use glam::Vec3;
use rand_distr::{Normal as Gaussian, Bernoulli, Distribution};

use crate::ray::{Ray, Optics};

use super::{color::Color, Material, normal::Normal};

#[derive(Clone, Copy)]
pub(crate) struct Glass {
    color: Color<f32>,
    rough: Gaussian<f32>,
    reflect: Bernoulli,
    refractive: f32,

    normal: Normal
}

impl Glass {
    pub(crate) fn new(
        color: Color<f32>,
        rough: f32,
        reflect: f64,
        refractive: f32
    ) -> Self {
        if color.r > 255.0 || color.g > 255.0 || color.b > 255.0 {
            panic!("Glass color value must be less than 255.0.");
        }

        let normalized_color = color / 255.0;

        Glass {
            color: normalized_color,
            rough: Gaussian::new(0.0, rough).unwrap(),
            reflect: Bernoulli::new(reflect).unwrap(),
            refractive,
            normal: Normal::new(color, rough)
        }
    }
}

impl Material for Glass {
    fn reflect(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray {
        if self.reflect.sample(&mut rand::thread_rng()) {
            self.normal.reflect(ray, normal, point)
        } else {
            self.refraction(ray, normal, point)
        }
    }
}

impl Glass {
    fn refraction(&self, ray: Ray, normal: Vec3, point: Vec3) -> Ray {
        let is_from_outside = Self::is_from_outside(ray, normal);

        let refractive_ratio = if is_from_outside {
            1.0 / self.refractive
        } else {
            self.refractive
        };

        let mut input_angle = Vec3::angle_between(normal, ray.direction);
        if is_from_outside { input_angle -= PI/2.0; }

        let output_angle = Self::get_output_angle(input_angle, refractive_ratio);
        
        if output_angle.is_none() {
            return self.normal.reflect(ray, normal, point);
        }

        let output_angle = output_angle.unwrap();

        let rotate_axis = Vec3::cross(ray.direction, normal);
        let mut diff_angle = output_angle - input_angle;
        if !is_from_outside { diff_angle *= -1.0; }

        let direction = ray.direction
                .rotate_from_axis_angle(rotate_axis, diff_angle)
                .dispersion(self.rough);

        let source = point;
        let color = ray.color * self.color;
        let reached_light = ray.reached_light;
        let reflect_count = ray.reflect_count
            + if is_from_outside { 1 } else { 0 };

        Ray {
            source,
            direction,
            color,
            reached_light,
            reflect_count
        }
    }

    fn is_from_outside(ray: Ray, normal: Vec3) -> bool {
        normal.dot(ray.direction) < 0.0
    }

    fn get_output_angle(input_angle: f32, refractive: f32) -> Option<f32> {
        let sin = input_angle.sin() * refractive;

        if sin.abs() > 1.0 {
            None
        } else {
            Some(sin.asin())
        }
    }
}