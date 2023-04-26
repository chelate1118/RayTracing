use glam::Vec3;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{map::world::World, ray::{RayColor, Ray}};

pub(crate) struct Camera {
    pub(crate) position: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) distance: f32,
    pub(crate) screen_unit_x: Vec3,
    pub(crate) screen_unit_y: Vec3
}

impl Camera {
    pub(crate) fn from_value(value: Value) -> Result<Self, serde_json::Error> {
        let ci: CameraInfo = serde_json::from_value(value)?;

        let pi = ci.pi.to_radians();
        let theta = ci.theta.to_radians();

        Ok(Camera {
            position: Vec3::new(ci.x, ci.y, ci.z),
            direction: Self::get_direction(ci),
            width: ci.width,
            height: ci.height,
            distance: ci.distance,
            screen_unit_x: Self::get_unit_x(ci),
            screen_unit_y: Self::get_unit_y(ci)
        })
    }

    pub(crate) fn start_ray(
        &self,
        world: World,
        x: usize,
        y: usize,
        ray_color: RayColor
    ) -> RayColor {
        world.start_ray(self.generate_ray(x, y, ray_color)).color
    }

    fn generate_ray(
        &self,
        x: usize,
        y: usize,
        ray_color: RayColor
    ) -> Ray {
        let x_pix_dist = (x as i32 - (self.width/2) as i32) as f32 + 0.5;
        let y_pix_dist = (y as i32 - (self.height/2) as i32) as f32 + 0.5;

        let x_dist = x_pix_dist / self.distance * self.screen_unit_x;
        let y_dist = y_pix_dist / self.distance * self.screen_unit_y;

        let direction = self.direction + x_dist + y_dist;

        Ray::new(self.position, direction, ray_color)
    }

    fn get_direction(ci: CameraInfo) -> Vec3 {
        let pi = ci.pi.to_radians();
        let theta = ci.theta.to_radians();

        Vec3::new(
            pi.cos() * theta.sin(),
            pi.sin() * theta.sin(),
            theta.cos()
        )
    }

    fn get_unit_x(ci: CameraInfo) -> Vec3 {
        Vec3::cross(
            Self::get_direction(ci),
            Vec3::Z
        ).normalize()
    }

    fn get_unit_y(ci: CameraInfo) -> Vec3 {
        Vec3::cross(
            Self::get_direction(ci),
            Self::get_unit_x(ci),
        ).normalize()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct CameraInfo {
    x: f32,
    y: f32,
    z: f32,
    pi: f32,
    theta: f32,
    width: usize,
    height: usize,
    distance: f32
}

impl CameraInfo {
    fn from_str(json: &str) -> Self {
        let ret: CameraInfo = serde_json::from_str(json).unwrap();
        
        assert!(ret.width % 2 == 0);
        assert!(ret.height % 2 == 0);

        ret
    }
}