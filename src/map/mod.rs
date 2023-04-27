pub(crate) mod world;
pub(crate) mod camera;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use world::World;
use camera::Camera;

use crate::{loader::FromValue, material::color::Color, ray::RayColor};

pub(crate) struct Map {
    camera: Camera,
    world: World
}

impl FromValue for Map {
    fn from_value(value: Value) -> serde_json::Result<Self>{
        let mi: MapInfo = serde_json::from_value(value)?;

        Ok(Map {
            camera: Camera::from_value(mi.camera)?,
            world: World::from_value(mi.object)?
        })
    }
}

impl Map {
    pub(crate) fn render_one_step(&self, screen: &mut[Vec<Color<i32>>]) {
        for (x, color_col) in screen.iter_mut().enumerate().take(self.camera.width) {
            for (y, color) in color_col.iter_mut().enumerate().take(self.camera.height) {
                *color += Color {
                    r: self.camera.start_ray(&self.world, x, y, RayColor::RED).get_value() as i32,
                    g: self.camera.start_ray(&self.world, x, y, RayColor::GREEN).get_value() as i32,
                    b: self.camera.start_ray(&self.world, x, y, RayColor::BLUE).get_value() as i32,
                }
            }
        }
        /*for x in 0..self.camera.width {
            for y in 0..self.camera.height {
                screen[x][y] += Color {
                    r: self.camera.start_ray(&self.world, x, y, RayColor::RED).get_value() as i32,
                    g: self.camera.start_ray(&self.world, x, y, RayColor::GREEN).get_value() as i32,
                    b: self.camera.start_ray(&self.world, x, y, RayColor::BLUE).get_value() as i32,
                }
            }
        }*/
    }
}

#[derive(Serialize, Deserialize)]
struct MapInfo {
    camera: Value,
    object: Value
}