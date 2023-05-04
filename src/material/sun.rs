use crate::object::sun::Sun;

use super::Material;

impl Material for Sun {
    fn reflect(&self, ray: crate::ray::Ray, _: glam::Vec3, _: glam::Vec3) -> crate::ray::Ray {
        crate::ray::Ray {
            source: glam::Vec3::ZERO,
            direction: glam::Vec3::ZERO,
            color: ray.color * self.get_surface_color(ray.direction),
            reached_light: true,
            reflect_count: ray.reflect_count + 1
        }
    }
}

impl Sun {
    fn get_surface_color(&self, direction: glam::Vec3) -> crate::material::color::Color<f32> {
        self.color * self.get_bright(direction)
    }

    fn get_bright(&self, direction: glam::Vec3) -> f32 {
        crate::math::normal_distribution(
            glam::Vec3::angle_between(-self.direction, direction),
            0.0,
            self.dispertion
        )
    }
}