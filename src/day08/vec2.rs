use std::ops;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::vec2::Vec2;

    #[test]
    fn test_math() {
        assert_eq!(
            Vec2::new(1.0, 2.0) + Vec2::new(2.0, 3.0),
            Vec2::new(3.0, 5.0)
        );
        assert_eq!(Vec2::new(1.0, 2.0) * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(Vec2::new(5.0, 4.0) / 2.0, Vec2::new(2.5, 2.0));
    }
}
