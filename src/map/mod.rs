pub(crate) mod world;
pub(crate) mod camera;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use world::World;
use camera::Camera;

pub(crate) struct Map {
    camera: Camera,
    world: World
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MapInfo {
    camera: Value,
    object: Value
}