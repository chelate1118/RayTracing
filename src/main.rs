#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod loader;
mod map;
mod export;
mod math;
mod test;

use std::fs;

use loader::FromValue;

pub struct HyperParameter {}
impl HyperParameter {
    const REFLECTION_COUNT: u32 = 12;
    const RENDER_COUNT: usize = 1024;
    const EXPORT_FRAME: usize = 8;
    const FILE_NAME: &str = "map2";
}

fn main() {
    fs::create_dir_all(format!("render/{}", HyperParameter::FILE_NAME)).expect("Fail to create directory.");

    let map = map::Map::from_value(
        loader::file_to_value(&format!("maps/{}.json", HyperParameter::FILE_NAME))
            .unwrap_or_else(|_| panic!("File {} doesn't exist.", HyperParameter::FILE_NAME))
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
        if (frame+1) % HyperParameter::EXPORT_FRAME == 0 {
            let cnt = frame / HyperParameter::EXPORT_FRAME;
            let file_name = format!("render/{}/render_{}.png", HyperParameter::FILE_NAME,cnt);

            export::screen_to_png(&screen, width, height, frame as i32, &file_name);
        }

        println!("process {}...", frame);
        map.render_one_step(&mut screen);
    }
}
