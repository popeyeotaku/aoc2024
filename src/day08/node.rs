use super::vec2::Vec2;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub code: char,
    pub pos: Vec2,
}

impl Node {
    #[inline]
    pub fn new<T>(code: char, x: T, y: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            code,
            pos: Vec2::new(x.into(), y.into()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AntiNode {
    pub x: u16,
    pub y: u16,
}

impl AntiNode {
    pub fn new(node: Node, width: u16, height: u16) -> Option<Self> {
        let x = node.pos.x.round();
        let y = node.pos.y.round();
        if x >= 0.0 && x < (width as f64) && y >= 0.0 && y < (height as f64) {
            Some(Self {
                x: x as u16,
                y: y as u16,
            })
        } else {
            None
        }
    }
}
