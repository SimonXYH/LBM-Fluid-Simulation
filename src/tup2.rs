use crate::vec2::Vec2;
use std::ops::{Add, Div, Mul, Neg, Sub};
use palette::num::Sqrt;

#[derive(Copy, Clone, Debug)]
pub struct Tup2<T>(pub T, pub T);

impl<T> Tup2<T> {
    pub fn new(x: T, y: T) -> Self {
        Tup2(x, y)
    }
    pub fn to_vec2(self) -> Vec2<T> {
        Vec2::new(self.0, self.1)
    }
}

impl Default for Tup2<f32> {
    fn default() -> Self {
        Tup2(0., 0.)
    }
}

impl Tup2<f32> {
    pub fn dot(self, other: Tup2<f32>) -> f32 {
        self.0 * other.0 + self.1 * other.1
    }
    pub fn mag_sq(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }
    
    pub fn mag(self) -> f32{
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    pub fn normalize(self) -> Self{
        self / self.mag()
    }

    pub fn to_usize(self) -> Tup2<usize> {
        Tup2(self.0 as usize, self.1 as usize)
    }
    pub fn floor(self) -> Tup2<usize> {
        Tup2((self.0).floor() as usize, (self.1).floor() as usize)
    }
}

impl Tup2<usize> {
    pub fn to_f32(self) -> Tup2<f32> {
        Tup2(self.0 as f32, self.1 as f32)
    }
}

impl<T: Add<Output = T>> Add for Tup2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tup2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for Tup2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tup2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Neg<Output = T>> Neg for Tup2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tup2(-self.0, -self.1)
    }
}

impl<T: Mul<Output = T>> Mul for Tup2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Tup2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T: Mul<f32, Output = T>> Mul<f32> for Tup2<T> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tup2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div<f32, Output = T>> Div<f32> for Tup2<T> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tup2(self.0 / rhs, self.1 / rhs)
    }
}
