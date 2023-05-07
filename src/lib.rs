mod object;
mod ray;
mod material;
mod loader;
mod map;
mod export;
mod math;
mod test;

use std::{fs, env, io::Write};

pub fn test() {
    let start_time = std::time::Instant::now();
    let mut before_time = start_time;
    let file_name = env::args().last().unwrap();
    let map = loader::load_map(&file_name);
    let mut screen = map.blank_screen();

    fs::create_dir_all(format!("render/{file_name}")).expect("Fail to create directory.");

    for sample in 1..map.config.render_count+1 {
        print!("process {}...", sample);
        std::io::stdout().flush().unwrap();
        map.render_one_step(&mut screen);

        if sample % map.config.export_frame == 0 {
            let file_name = 
                if cfg!(feature = "bmp") {
                    format!("render/{file_name}/render_{sample}.bmp")
                }
                else {
                    format!("render/{file_name}/render_{sample}.png")
                };

            export::screen_to_png(&screen, sample as i32, &file_name);
        }

        let elapsed = before_time.elapsed().as_secs_f32();
        let sample_per_sec = start_time.elapsed().as_secs_f32() / sample as f32;

        println!("  done. ({:.3}s) - {:.3} sample/sec", elapsed, sample_per_sec);
        before_time = std::time::Instant::now();
    }
}
