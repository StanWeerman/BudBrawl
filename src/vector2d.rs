use std::{cell::Ref, ops};

#[derive(Clone)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl ops::Add<&Vector2d> for &Vector2d {
    type Output = Vector2d;
    fn add(self, other_vec: &Vector2d) -> Self::Output {
        Vector2d::new(self.x + other_vec.x, self.y + other_vec.y)
    }
}
impl ops::AddAssign<Vector2d> for Vector2d {
    fn add_assign(&mut self, other_vec: Vector2d) {
        self.x += other_vec.x;
        self.y += other_vec.y;
    }
}
impl ops::AddAssign<&Vector2d> for Vector2d {
    fn add_assign(&mut self, other_vec: &Vector2d) {
        self.x += other_vec.x;
        self.y += other_vec.y;
    }
}
impl ops::AddAssign<&Ref<'_, Vector2d>> for Vector2d {
    fn add_assign(&mut self, other_vec: &Ref<'_, Vector2d>) {
        self.x += other_vec.x;
        self.y += other_vec.y;
    }
}
impl ops::Sub<&Vector2d> for &Vector2d {
    type Output = Vector2d;
    fn sub(self, other_vec: &Vector2d) -> Self::Output {
        Vector2d::new(self.x - other_vec.x, self.y - other_vec.y)
    }
}
impl ops::Sub<&Ref<'_, Vector2d>> for &Vector2d {
    type Output = Vector2d;
    fn sub(self, other_vec: &Ref<'_, Vector2d>) -> Self::Output {
        Vector2d::new(self.x - other_vec.x, self.y - other_vec.y)
    }
}
impl ops::Sub<&Vector2d> for &Ref<'_, Vector2d> {
    type Output = Vector2d;
    fn sub(self, other_vec: &Vector2d) -> Self::Output {
        Vector2d::new(self.x - other_vec.x, self.y - other_vec.y)
    }
}
impl ops::MulAssign<Vector2d> for Vector2d {
    fn mul_assign(&mut self, other_vec: Vector2d) {
        self.x *= other_vec.x;
        self.y *= other_vec.y;
    }
}
impl ops::Mul<&Vector2d> for &Vector2d {
    type Output = Vector2d;
    fn mul(self, other_vec: &Vector2d) -> Self::Output {
        Vector2d::new(self.x * other_vec.x, self.y * other_vec.y)
    }
}
