use std::{ops::{Div, Mul, AddAssign}, cmp::min};

use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Default)]
pub(crate) struct Color<T> {
    pub(crate) r: T,
    pub(crate) g: T,
    pub(crate) b: T
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
    pub(crate) fn new(r: T, g: T, b: T) -> Self {
        Color { r, g, b }
    }

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