use image::{RgbImage, ImageBuffer};

use crate::material::color::Color;

pub(super) fn screen_to_png(
    screen: &[Vec<Color<i32>>],
    width: usize,
    height: usize,
    scale: i32,
    file_name: &str
) {
    let img: RgbImage = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        color_to_rgb(screen[y as usize][x as usize], scale)
    });

    img.save(file_name).unwrap();
}

fn color_to_rgb(color: Color<i32>, scale: i32) -> image::Rgb<u8> {
    let color = (color / scale).limit();
    image::Rgb([color.r as u8, color.g as u8, color.b as u8])
}