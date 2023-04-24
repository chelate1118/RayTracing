use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Camera {
    x: f32,
    y: f32,
    z: f32,
    pi: f32,
    theta: f32,
    width: usize,
    height: usize
}

impl Camera {
    
}