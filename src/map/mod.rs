pub(crate) mod world;
pub(crate) mod camera;
pub(crate) mod config;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use world::World;
use camera::Camera;
use config::Config;

use crate::{loader::FromValue, material::color::Color, ray::RayColor};

pub(crate) struct Map {
    pub(crate) config: Config,
    pub(crate) camera: Camera,
    pub(crate) world: World
}

impl FromValue for Map {
    fn from_value(value: Value) -> serde_json::Result<Self>{
        let mi: MapInfo = serde_json::from_value(value)?;

        Ok(Map {
            config: Config::from_value(mi.config)?,
            camera: Camera::from_value(mi.camera)?,
            world: World::from_value(mi.object)?
        })
    }
}

impl Map {
    pub(crate) fn render_one_step(&self, screen: &mut[Vec<Color<i32>>]) {
        for (y, color_col) in screen.iter_mut().enumerate() {
            for (x, color) in color_col.iter_mut().enumerate() {
                *color += self.get_pixel_color(x, y);
            }
        }
    }

    pub(crate) fn get_pixel_color(&self, x: usize, y: usize) -> Color<i32> {
        Color {
            r: self.camera.start_ray(
                &self.world, x, y,
                RayColor::RED,
                self.config.reflect_count
            ).get_value(self.config.bright) as i32,
            g: self.camera.start_ray(
                &self.world, x, y,
                RayColor::GREEN,
                self.config.reflect_count
            ).get_value(self.config.bright) as i32,
            b: self.camera.start_ray(
                &self.world, x, y,
                RayColor::BLUE,
                self.config.reflect_count
            ).get_value(self.config.bright) as i32,
        }
    }

    pub(crate) fn blank_screen(&self) -> Vec<Vec<Color<i32>>> {
        let (width, height) = (self.camera.width, self.camera.height);

        vec![ vec![Color::<i32>::default(); width]; height]
    }
}

#[derive(Serialize, Deserialize)]
struct MapInfo {
    config: Value,
    camera: Value,
    object: Value
}