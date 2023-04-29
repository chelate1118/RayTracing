#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod loader;
mod map;
mod export;
mod test;

use loader::FromValue;

pub struct HyperParameter {}
impl HyperParameter {
    const REFLECTION_COUNT: u32 = 3;
    const RENDER_COUNT: usize = 1024;
    const EXPORT_FRAME: usize = 32;
    const FILE_PATH: &str = "maps/map1.json";
}

fn main() {
    let map = map::Map::from_value(
        loader::file_to_value(HyperParameter::FILE_PATH)
            .unwrap_or_else(|_| panic!("File {} doesn't exist.", HyperParameter::FILE_PATH))
    ).expect("Fail to deserialize map informations.");

    let (width, height) = map.get_screen_size();

    let mut screen = vec![
        vec![
            material::color::Color::<i32>::default();
            width
        ];
        height
    ];

    for frame in 0..HyperParameter::RENDER_COUNT {
        if frame % HyperParameter::EXPORT_FRAME == 0 && frame > 0 {
            let cnt = frame / HyperParameter::EXPORT_FRAME;
            let file_name = format!("render/render_{}.png", cnt);

            export::screen_to_png(&screen, width, height, frame as i32, &file_name);
        }

        println!("process {}...", frame);
        map.render_one_step(&mut screen);
    }
}
