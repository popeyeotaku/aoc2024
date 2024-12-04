use std::fs;

use find::{count_mas, count_xmas};
use grid::Grid;

pub fn day04() {
    let input = fs::read_to_string("day04_input.txt").unwrap();
    let grid = Grid::new(&input);
    let count = count_xmas(&grid);
    println!("XMAS count: {}", count);

    let count = count_mas(&grid);
    println!("MAS count: {}", count);
}

mod find {

    use super::{
        dir::{Dir, Line, ALL_DIRS},
        grid::Grid,
    };

    const XMAS: &str = "XMAS";

    pub fn count_mas(grid: &Grid) -> u64 {
        let mut count: u64 = 0;

        for y in 0..grid.height() {
            for x in 0..grid.width {
                if find_mas(grid, (x, y)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn find_mas(grid: &Grid, start_coord: (usize, usize)) -> bool {
        let (x, y) = start_coord;
        let a_coord = (x + 1, y + 1);
        if grid.valid(a_coord) && grid.get(a_coord) == 'A' {
            let nw = start_coord;
            let ne = (x + 2, y);
            let sw = (x, y + 2);
            let se = (x + 2, y + 2);
            if [nw, ne, sw, se].iter().copied().all(|c| grid.valid(c)) {
                let nw = grid.get(nw);
                let ne = grid.get(ne);
                let sw = grid.get(sw);
                let se = grid.get(se);
                ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
                    && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn count_xmas(grid: &Grid) -> u64 {
        let mut count: u64 = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width {
                count += find_coord(grid, (x, y)) as u64;
            }
        }
        count
    }

    fn find_coord(grid: &Grid, coord: (usize, usize)) -> u8 {
        let mut count = 0;
        for dir in ALL_DIRS {
            if find_str(grid, XMAS, coord, dir) {
                count += 1;
            }
        }
        count
    }

    pub fn find_str(grid: &Grid, s: &str, coord: (usize, usize), dir: Dir) -> bool {
        if let Some(line) = Line::new(grid, coord, dir, s.chars().count()) {
            for (c, coord) in s.chars().zip(line.coords.iter()) {
                let grid_c = grid.get(*coord);
                if grid_c != c {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

mod dir {
    use super::grid::Grid;

    #[derive(Clone, Copy)]
    pub enum Dir {
        N,
        S,
        E,
        W,
        NW,
        NE,
        SW,
        SE,
    }

    pub const ALL_DIRS: [Dir; 8] = [
        Dir::N,
        Dir::S,
        Dir::E,
        Dir::W,
        Dir::NW,
        Dir::NE,
        Dir::SW,
        Dir::SE,
    ];

    impl Dir {
        pub fn vector(self) -> (i8, i8) {
            match self {
                Dir::N => (0, -1),
                Dir::S => (0, 1),
                Dir::E => (1, 0),
                Dir::W => (-1, 0),
                Dir::NW => (-1, -1),
                Dir::NE => (1, -1),
                Dir::SW => (-1, 1),
                Dir::SE => (1, 1),
            }
        }
    }

    pub struct Line {
        pub coords: Vec<(usize, usize)>,
    }

    impl Line {
        pub fn new(grid: &Grid, start: (usize, usize), dir: Dir, len: usize) -> Option<Self> {
            let mut coords = vec![start];
            for _ in 1..len {
                if let Some(new_coord) = grid.coord(*coords.last().unwrap(), dir) {
                    coords.push(new_coord);
                } else {
                    return None;
                }
            }
            debug_assert_eq!(coords.len(), len);
            Some(Self { coords })
        }
    }
}

mod grid {
    use super::dir::Dir;

    struct GridChars<'a> {
        grid: &'a Grid,
        coord: (usize, usize),
    }

    impl<'a> GridChars<'a> {
        pub fn new(grid: &'a Grid) -> Self {
            Self {
                grid,
                coord: (0, 0),
            }
        }
    }

    impl Iterator for GridChars<'_> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            let (x, y) = self.coord;
            if x >= self.grid.width {
                self.coord = (0, y + 1);
                Some('\n')
            } else if y >= self.grid.height() {
                None
            } else {
                let c = self.grid.get(self.coord);
                self.coord = (x + 1, y);
                Some(c)
            }
        }
    }

    pub struct Grid {
        pub width: usize,
        elems: Vec<char>,
    }

    impl Grid {
        pub fn new(s: &str) -> Self {
            let width = s.lines().next().unwrap().chars().count();
            let mut elems = Vec::with_capacity(width * s.lines().size_hint().0);
            for line in s.lines() {
                assert_eq!(line.len(), width);
                elems.extend(line.chars());
            }
            let grid = Self { width, elems };
            let grid_s = String::from_iter(GridChars::new(&grid));
            assert_eq!(&grid_s, s);
            grid
        }

        pub fn height(&self) -> usize {
            assert_eq!(self.elems.len() % self.width, 0);
            self.elems.len() / self.width
        }

        pub fn valid(&self, coord: (usize, usize)) -> bool {
            let (x, y) = coord;
            x < self.width && y < self.height()
        }

        pub fn get(&self, coords: (usize, usize)) -> char {
            assert!(self.valid(coords));
            let (x, y) = coords;
            self.elems[y * self.width + x]
        }

        pub fn coord(&self, coord: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
            let (x1, y1) = coord;
            let (dx, dy) = dir.vector();
            let x2 = (x1 as isize) + (dx as isize);
            let y2 = (y1 as isize) + (dy as isize);
            if x2 >= 0 && x2 < (self.width as isize) && y2 >= 0 && y2 < (self.height() as isize) {
                let x2 = x2 as usize;
                let y2 = y2 as usize;
                Some((x2, y2))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day04::{dir::Dir, find::find_str};

    use super::grid::Grid;

    #[test]
    fn test_grid() {
        let s = "123\n456\n789";
        let grid = Grid::new(s);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height(), 3);
        let mut nums = "123456789".chars();
        for y in 0..3 {
            for x in 0..3 {
                let num = nums.next().unwrap();
                let c = grid.get((x, y));
                assert_eq!(c, num);
            }
        }

        assert!(find_str(&grid, "123", (0, 0), Dir::E));
        assert!(find_str(&grid, "147", (0, 0), Dir::S));
        assert!(find_str(&grid, "951", (2, 2), Dir::NW));
    }
}
