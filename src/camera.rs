use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug)]
pub(crate) struct Camera {
    pub(crate) position: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) width: usize,
    pub(crate) height: usize
}

impl Camera {
    pub(crate) fn from_str(json: &str) -> Self {
        let ci = CameraInfo::from_str(json);

        let pi = ci.pi.to_radians();
        let theta = ci.theta.to_radians();

        Camera {
            position: Vec3::new(ci.x, ci.y, ci.z),
            direction: Vec3::new(
                pi.cos() * theta.sin(),
                pi.sin() * theta.sin(),
                theta.cos()
            ),
            width: ci.width,
            height: ci.height
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CameraInfo {
    x: f32,
    y: f32,
    z: f32,
    pi: f32,
    theta: f32,
    width: usize,
    height: usize
}

impl CameraInfo {
    fn from_str(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}