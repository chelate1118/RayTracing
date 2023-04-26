pub(crate) mod world;
pub(crate) mod camera;

use world::World;
use camera::Camera;

struct Map {
    camera: Camera,
    world: World
}