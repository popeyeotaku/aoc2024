use std::collections::HashSet;

use ray::Ray;

use super::{node::AntiNode, parse::parse, vec2::Vec2};

pub fn part2(s: &str) -> HashSet<AntiNode> {
    let (nodes, width, height) = parse(s);
    let mut antinodes: HashSet<AntiNode> = HashSet::new();
    for a in &nodes {
        for b in &nodes {
            if a != b && a.code == b.code {
                let ray = Ray::new(a.pos, b.pos);
                for y in 0..height {
                    for x in 0..width {
                        let pos = Vec2::new(x as f64, y as f64);
                        if ray.find_x(pos.x) == pos {
                            antinodes.insert(AntiNode::new(x, y));
                        }
                    }
                }
            }
        }
    }
    antinodes
}

mod ray;
