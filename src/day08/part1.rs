use std::collections::HashSet;

use super::{
    node::{AntiNode, Node},
    parse::parse,
};

pub type Out = HashSet<AntiNode>;

pub fn part1(s: &str) -> Out {
    let (nodes, width, height) = parse(s);
    let mut antinodes: HashSet<AntiNode> = HashSet::new();
    for a in &nodes {
        for b in &nodes {
            if a.code == b.code && a != b {
                let normal = a.pos.relative(b.pos).normal();
                let dist = a.pos.dist(b.pos) * 2.0;
                let pos = normal * dist + a.pos;
                #[cfg(debug_assertions)]
                {
                    let a_dist = pos.dist(a.pos);
                    let b_dist = pos.dist(b.pos);
                    assert_eq!((a_dist - 2.0 * b_dist).abs().floor(), 0.0);
                }
                if let Some(antinode) =
                    AntiNode::new(Node::new(a.code, pos.x, pos.y), width, height)
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes
}
