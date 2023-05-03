use std::f32::consts::{PI, E};

pub(crate) fn normal_distribution(x: f32, mean: f32, sigma: f32) -> f32 {
    1.0 / sigma / (2.0 * PI).sqrt() * E.powf(-0.5 * ((x - mean) / sigma).powi(2))
}