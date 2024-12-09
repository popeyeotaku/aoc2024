use crate::day08::vec2::Vec2;

pub struct Ray {
    start: Vec2,
    dir: Vec2,
}

impl Ray {
    pub fn new(start: Vec2, stop: Vec2) -> Self {
        let dir = start.relative(stop).normal();
        Self { start, dir }
    }

    pub fn cast(&self, len: f64) -> Vec2 {
        self.dir * len + self.start
    }

    fn find_x_dist(&self, x: f64) -> f64 {
        // self.dir.x * dist + self.start.x = x
        // self.dir.x * dist = x - self.start.x
        // (x - self.start.x) / self.dir.x
        if self.dir.x == 0.0 {
            0.0
        } else {
            (x - self.start.x) / self.dir.x
        }
    }

    pub fn find_x(&self, x: f64) -> Vec2 {
        self.cast(self.find_x_dist(x))
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::vec2::Vec2;

    use super::Ray;

    #[test]
    fn test_cast() {
        let ray = Ray::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0));
        assert_eq!(ray.cast(3.0), Vec2::new(3.0, 0.0));
    }

    #[test]
    fn test_find() {
        let ray = Ray::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0));
        assert_eq!(ray.find_x(3.0), Vec2::new(3.0, 0.0));
    }
}
