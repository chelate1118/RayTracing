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

fn main() {
    let file_name = env::args().last().expect("File name not provided.");
    fs::create_dir_all(format!("render/{file_name}")).expect("Fail to create directory.");

    let map = loader::load_map(&file_name);
    let mut screen = map.blank_screen();

    for frame in 1..map.config.render_count+1 {
        print!("process {}...", frame);
        map.render_one_step(&mut screen);

        if frame % map.config.export_frame == 0 {
            let file_name = format!("render/{file_name}/render_{frame}.png");

            export::screen_to_png(&screen, frame as i32, &file_name);
        }

        println!("  done.");
    }
}
