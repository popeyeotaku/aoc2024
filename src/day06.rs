use std::fs;

use guard::Guard;
use map::{Map, Pos, Square};

pub fn day06() {
    let input = fs::read_to_string("day06_input.txt").unwrap();
    let (map, guard) = map::load_map(&input);
    let visited_squares = guard.clone().find_visited(&map).unwrap();
    println!("visited squares: {}", visited_squares.len());

    let loops = find_stucks(&map, &guard);
    println!("possible stucks: {}", loops);
}

fn find_stucks(map: &Map, guard: &Guard) -> usize {
    let mut loops: usize = 0;
    for x in 0..map.width {
        for y in 0..map.height() {
            let pos = Pos { x, y: y as u16 };
            if Some(pos) == guard.pos {
                continue;
            }
            if map.get(pos) == Some(Square::Empty) {
                let mut new_map = map.clone();
                new_map.put(pos, Square::Obstacle);
                if guard.clone().find_visited(&new_map).is_none() {
                    loops += 1;
                }
            }
        }
    }
    loops
}

mod guard {
    use std::collections::HashSet;

    use super::{
        dir::Dir,
        map::{Map, Pos, Square},
    };

    #[derive(Clone)]
    pub struct Guard {
        pub pos: Option<Pos>,
        dir: Dir,
    }

    impl Guard {
        pub fn new(pos: Pos, dir: Dir) -> Self {
            Self {
                pos: Some(pos),
                dir,
            }
        }

        pub fn step(&mut self, map: &Map) {
            let old_pos = self.pos.unwrap();
            let new_pos = old_pos + self.dir;
            debug_assert_eq!(map.get(old_pos), Some(Square::Empty));
            match map.get(new_pos) {
                Some(Square::Empty) => self.pos = Some(new_pos),
                Some(Square::Obstacle) => self.dir = self.dir.rotate(),
                None => self.pos = None,
            }
        }

        pub fn find_visited(&mut self, map: &Map) -> Option<HashSet<Pos>> {
            let mut total_steps: usize = 1;
            let mut visited: HashSet<Pos> = HashSet::new();
            while let Some(cur_pos) = self.pos {
                if total_steps >= map.total_squares() {
                    return None;
                }
                visited.insert(cur_pos);
                self.step(map);
                if self.pos != Some(cur_pos) {
                    total_steps += 1;
                }
            }
            Some(visited)
        }
    }
}

mod dir {
    #[derive(Clone, Copy)]
    pub enum Dir {
        N,
        S,
        E,
        W,
    }

    impl Dir {
        pub fn vector(self) -> (i8, i8) {
            match self {
                Dir::N => (0, -1),
                Dir::S => (0, 1),
                Dir::E => (1, 0),
                Dir::W => (-1, 0),
            }
        }

        pub fn rotate(self) -> Self {
            match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::W => Dir::N,
                Dir::S => Dir::W,
            }
        }
    }
}

mod map {
    use std::ops;

    use super::{dir::Dir, guard::Guard};

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Pos {
        pub x: u16,
        pub y: u16,
    }

    impl ops::Add<(i8, i8)> for Pos {
        type Output = Self;

        fn add(self, rhs: (i8, i8)) -> Self::Output {
            let (x1, y1) = (self.x, self.y);
            let (x2, y2) = rhs;
            let x = ((x1 as i16) + (x2 as i16)) as u16;
            let y = ((y1 as i16) + (y2 as i16)) as u16;
            Self { x, y }
        }
    }

    impl ops::Add<Dir> for Pos {
        type Output = Self;

        fn add(self, rhs: Dir) -> Self::Output {
            self + rhs.vector()
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Square {
        Empty,
        Obstacle,
    }

    #[derive(Clone)]
    pub struct Map {
        pub width: u16,
        elems: Vec<Square>,
    }

    impl Map {
        pub fn total_squares(&self) -> usize {
            self.elems.len()
        }

        pub fn height(&self) -> usize {
            self.elems.len() / (self.width as usize)
        }

        pub fn get(&self, pos: Pos) -> Option<Square> {
            if pos.x < self.width && pos.y < (self.height() as u16) {
                Some(self.elems[(pos.y as usize) * self.height() + (pos.x as usize)])
            } else {
                None
            }
        }

        pub fn put(&mut self, pos: Pos, square: Square) {
            let height = self.height();
            if pos.x < self.width && pos.y < (height as u16) {
                self.elems[(pos.y as usize) * height + (pos.x as usize)] = square;
            } else {
                panic!("bad coords")
            }
        }
    }

    pub fn load_map(s: &str) -> (Map, Guard) {
        let mut guard: Option<Guard> = None;
        let width = s.lines().next().unwrap().chars().count();
        let mut elems: Vec<Square> = Vec::with_capacity(s.lines().size_hint().0 * width);
        for (y, line) in s.lines().enumerate() {
            assert_eq!(line.chars().count(), width);
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => elems.push(Square::Empty),
                    '#' => elems.push(Square::Obstacle),
                    '^' => {
                        assert!(guard.is_none());
                        guard = Some(Guard::new(
                            Pos {
                                x: x as u16,
                                y: y as u16,
                            },
                            Dir::N,
                        ));
                        elems.push(Square::Empty);
                    }
                    _ => panic!("unexpected character '{}'", c),
                }
            }
        }
        (
            Map {
                width: width as u16,
                elems,
            },
            guard.unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::day06::{
        find_stucks,
        map::{Pos, Square},
    };

    use super::map::load_map;

    #[test]
    fn test_input() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        let (map, guard) = load_map(input);
        assert_eq!(map.get(Pos { x: 9, y: 1 }), Some(Square::Obstacle));
        let visited = guard.clone().find_visited(&map).unwrap();
        assert_eq!(visited.len(), 41);
        let stucks = find_stucks(&map, &guard);
        assert_eq!(stucks, 6);
    }
}