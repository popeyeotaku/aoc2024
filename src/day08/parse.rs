use std::{iter::Peekable, str::Chars};

use super::node::Node;

/// world: line+ trailer
/// line: elem+ '\n'
/// elem: '.' | code
/// trailer: '\n'*
pub fn parse(s: &str) -> (Vec<Node>, u16, u16) {
    let width = s.lines().next().unwrap().chars().count() as u16;
    let mut s = s.chars().peekable();
    let mut nodes: Vec<Node> = Vec::from_iter(line(&mut s, 0).drain(0..).flatten());
    let mut y = 1;
    while s.peek().map(|c| *c != '\n').unwrap_or(false) {
        nodes.extend(line(&mut s, y).drain(0..).flatten());
        y += 1;
    }
    trailer(&mut s);
    (nodes, width, y)
}

fn line(s: &mut Peekable<Chars<'_>>, y: u16) -> Vec<Option<Node>> {
    let mut nodes: Vec<Option<Node>> = vec![elem(s, 0, y)];
    let mut x = 1;
    while s.peek() != Some(&'\n') {
        nodes.push(elem(s, x, y));
        x += 1;
    }
    assert_eq!(s.next().unwrap(), '\n');
    nodes
}

fn elem(s: &mut Peekable<Chars<'_>>, x: u16, y: u16) -> Option<Node> {
    let c = s.next().unwrap();
    assert_ne!(c, '\n');
    if c.is_ascii_alphabetic() || c.is_ascii_digit() {
        Some(Node::new(c, x, y))
    } else if c == '.' {
        None
    } else {
        panic!("bad character '{}'", c);
    }
}

fn trailer(s: &mut Peekable<Chars<'_>>) {
    for c in s {
        assert_eq!(c, '\n');
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::{node::Node, parse::parse};

    #[test]
    fn test_parse() {
        let input = "...
...
.a.
..B
0..
";
        let (nodes, width, height) = parse(input);
        assert_eq!(width, 3);
        assert_eq!(height, 5);
        assert_eq!(
            nodes,
            vec![
                Node::new('a', 1, 2),
                Node::new('B', 2, 3),
                Node::new('0', 0, 4)
            ]
        )
    }
}
