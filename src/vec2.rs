use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-6 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn distance_to(&self, other: &Vec2) -> f32 {
        (*self - *other).length()
    }

    pub fn distance_squared(&self, other: &Vec2) -> f32 {
        (*self - *other).length_squared()
    }

    pub fn perpendicular(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}
