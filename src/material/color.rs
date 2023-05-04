use std::{ops::{Div, Mul, AddAssign}, cmp::min};

use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub(crate) struct Color<T> {
    pub(crate) r: T,
    pub(crate) g: T,
    pub(crate) b: T
}

impl Color<f32> {
    pub(crate) fn to_color_i32(&self, scale: f32) -> Color<i32> {
        Color {
            r: (self.r * scale) as i32,
            g: (self.g * scale) as i32,
            b: (self.b * scale) as i32
        }
    }
}

impl Default for Color<f32> {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }
}

impl Default for Color<i32> {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

impl Color<i32> {
    pub(crate) fn limit(&self) -> Self {
        Color {
            r: min(self.r, 255),
            g: min(self.g, 255),
            b: min(self.b, 255)
        }
    }
}

impl<T> Color<T> {
    pub(crate) fn from_array(x: [T; 3]) -> Self
        where T: Copy {
        Color {
            r: x[0],
            g: x[1],
            b: x[2]
        }
    }
}

impl<T1, T2> AddAssign<Color<T1>> for Color<T2> where T2: AddAssign<T1> {
    fn add_assign(&mut self, rhs: Color<T1>) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<Color<f32>> for Color<f32> {
    type Output = Self;

    fn mul(self, rhs: Color<f32>) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
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

impl<T, D> Div<D> for Color<T> where T: Div<D, Output = T>, D: Copy {
    type Output = Self;

    fn div(self, rhs: D) -> Self {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}