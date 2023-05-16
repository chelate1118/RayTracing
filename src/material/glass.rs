use glam::{Vec3, Quat};
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

        let color = color / 255.0;

        Glass {
            color,
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
        let coeff = if Self::is_from_outside(ray, normal) {
            1.0 / self.refractive
        } else {
            self.refractive
        };

        let input_angle = Vec3::angle_between(normal, ray.direction);
        let output_angle = f32::asin(input_angle.sin() * coeff);
        let rotate_axis = Vec3::cross(ray.direction, normal);

        let diff_angle = input_angle - output_angle;

        let quat = Quat::from_axis_angle(rotate_axis, diff_angle);

        let mut direction = quat.mul_vec3(ray.direction);

        direction = direction.dispersion(self.rough);
        while is_invalid_refract(ray.direction, direction, normal) {
            direction = direction.dispersion(self.rough);
        }

        let source = point;
        let color = ray.color * self.color;
        let reached_light = ray.reached_light;
        let reflect_count = ray.reflect_count + 1;

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
}

fn is_invalid_refract(before: Vec3, after: Vec3, normal: Vec3) -> bool {
    before.dot(normal) * after.dot(normal) < 0.0
}