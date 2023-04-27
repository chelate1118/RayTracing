#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod loader;
mod map;
mod test;

use loader::FromValue;

pub struct HyperParameter {}
impl HyperParameter {
    const REFLECTION_COUNT: u32 = 2;
    const RENDER_COUNT: usize = 1024;
    const EXPORT_FRAME: usize = 128;

    const MAX_WIDTH: usize = 2100;
    const MAX_HEIGHT: usize = 2100;
    const FILE_PATH: &str = "maps/map1.json";
}

fn main() {
    let map = map::Map::from_value(
        loader::file_to_value(HyperParameter::FILE_PATH)
            .unwrap_or_else(|_| panic!("File {} doesn't exist.", HyperParameter::FILE_PATH))
    ).expect("Fail to deserialize map informations.");

    let mut screen = vec![
        vec![
            material::color::Color::<i32>::default();
            HyperParameter::MAX_WIDTH
        ];
        HyperParameter::MAX_HEIGHT
    ];

    for frame in 0..HyperParameter::RENDER_COUNT {
        map.render_one_step(screen.as_mut_slice());

        if frame % HyperParameter::EXPORT_FRAME == 0 {
            let cnt = frame / HyperParameter::EXPORT_FRAME;
            let file_name = format!("render_{}.png", cnt);

            todo!("Implement exporting image.")
        }
    }
}
