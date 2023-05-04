#![allow(unused_variables)]

mod object;
mod ray;
mod material;
mod loader;
mod map;
mod export;
mod math;
mod test;

use std::{fs, env};

use loader::FromValue;

fn main() {
    let file_name = env::args().last().expect("File name not provided.");

    fs::create_dir_all(format!("render/{file_name}")).expect("Fail to create directory.");

    let map = map::Map::from_value(
        loader::file_to_value(&format!("maps/{file_name}.json"))
            .unwrap_or_else(|_| panic!("File {file_name} doesn't exist."))
    ).expect("Fail to deserialize map informations.");

    let (width, height) = map.get_screen_size();

    let mut screen = vec![
        vec![
            material::color::Color::<i32>::default();
            width
        ];
        height
    ];

    for frame in 0..map.config.render_count {
        if (frame+1) % map.config.export_frame == 0 {
            let file_name = format!("render/{}/render_{}.png", map.config.file_name, frame+1);

            export::screen_to_png(&screen, width, height, frame as i32, &file_name);
        }

        println!("process {}...", frame+1);
        map.render_one_step(&mut screen);
    }
}
