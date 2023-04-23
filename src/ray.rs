#![allow(dead_code)]

use glam::Vec3;

#[derive(Clone, Copy)]
pub(crate) struct Ray {
    source: Vec3,
    direction: Vec3,
    bright: f32
}

impl Ray {
    pub fn new(source: Vec3, direction: Vec3, bright: f32) -> Self {
        return Ray { source, direction, bright };
    }

    pub fn get_source(&self) -> Vec3 {
        return self.source;
    }
    pub fn get_direction(&self) -> Vec3 {
        return self.direction.normalize();
    }
    pub fn get_bright(&self) -> f32 {
        return self.bright;
    }
}