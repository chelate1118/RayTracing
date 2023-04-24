#![allow(dead_code)]

#[derive(Clone, Copy)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32
}

impl Color {
    pub(crate) fn new(r: f32, g: f32, b: f32) -> Self {
        return Color { r, g, b }; 
    }
}