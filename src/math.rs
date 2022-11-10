use core::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn dot(&self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl From<i32> for Vec2 {
    fn from(val: i32) -> Self {
        let f = val as f32;
        Vec2::new(f, f)
    }
}
impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
