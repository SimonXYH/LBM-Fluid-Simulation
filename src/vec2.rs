use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::process::Output;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>{
    pub x: T,
    pub y: T
}

impl<T> Vec2<T>{
    pub fn new(x: T, y: T) -> Self{
        Vec2{
            x, y
        }
    }
}

impl Vec2<f32>{
    pub fn to_i32(self) -> Vec2<i32>{
        Vec2::new(self.x as i32, self.y as i32)
    }
    pub fn negate_y(self) -> Vec2<f32>{
        Vec2::new(self.x, - self.y)
    }
    
    pub fn dist(self, other: Vec2<f32>) -> f32{
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    
    pub fn dist_sq(self, other: Vec2<f32>) -> f32{
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    
    pub fn random(mag_range: (f32, f32)) -> Self{
        let mut rng = thread_rng();
        let mag = rng.gen_range(mag_range.0..mag_range.1);
        let angle = rng.gen_range(0f32..2. * PI);
        Vec2::new(angle.cos(), angle.sin()) * mag
    }
}

impl<T: Add<T, Output = T>> Add for Vec2<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<T, Output = T>> Sub for Vec2<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<T, Output = T>> Mul for Vec2<T>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl<T: Div<T, Output = T>> Div for Vec2<T>{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: Mul<f32, Output = T>> Mul<f32> for Vec2<T>{
    type Output = Vec2<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<f32, Output = T>> Div<f32> for Vec2<T>{
    type Output = Vec2<T>;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}


impl<T: Neg<Output = T>> Neg for Vec2<T>{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, - self.y)
    }
}