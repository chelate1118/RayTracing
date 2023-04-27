#![allow(dead_code)]

use std::ops::{Div, Mul, AddAssign};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub(crate) struct Color<T> {
    pub(crate) r: T,
    pub(crate) g: T,
    pub(crate) b: T
}

impl<T> Color<T> {
    pub(crate) fn new(r: T, g: T, b: T) -> Self {
        Color { r, g, b }
    }
}

impl<T1, T2> AddAssign<Color<T1>> for Color<T2> where T2: AddAssign<T1> {
    fn add_assign(&mut self, rhs: Color<T1>) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T> Mul<f32> for Color<T> where T: Mul<f32, Output = T> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl<T> Div<f32> for Color<T> where T: Div<f32, Output = T> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}