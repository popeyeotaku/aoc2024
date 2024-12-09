pub fn day08() {
    todo!()
}

mod vec2;

mod node {
    use super::vec2::Vec2;

    #[derive(PartialEq, Debug)]
    pub struct Node {
        code: char,
        pos: Vec2,
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
}

mod parse;

mod part1 {
    pub type Out = u32;

    pub fn part1(s: &str) -> Out {
        todo!()
    }
}

mod part2 {}

#[cfg(test)]
mod tests {
    use crate::day08::part1;

    #[test]
    fn test_sample() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(part1::part1(input), 14);
    }
}
