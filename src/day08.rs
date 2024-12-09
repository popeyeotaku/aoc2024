use std::fs;

pub fn day08() {
    let input = fs::read_to_string("day08_input.txt").unwrap();
    let sum = part1::part1(&input).len();
    println!("part 1: {}", sum);
    let sum = part2::part2(&input).len();
    println!("part 2: {}", sum);
}

mod vec2;

mod node;

mod parse;

mod part1;

mod part2;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day08::{
        part1,
        part2::{self},
    };

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
        let antinodes = part1::part1(input);
        let poslist: HashSet<[u16; 2]> = HashSet::from([
            [6, 0],
            [11, 0],
            [3, 1],
            [4, 2],
            [10, 2],
            [2, 3],
            [9, 4],
            [1, 5],
            [3, 6],
            [0, 7],
            [7, 7],
            [10, 10],
            [10, 11],
            [6, 5],
        ]);
        assert_eq!(poslist.len(), 14);
        assert_eq!(
            HashSet::from_iter(antinodes.iter().map(|a| [a.x, a.y])),
            poslist
        );
        assert_eq!(part2::part2(input).len(), 34);
    }
}
