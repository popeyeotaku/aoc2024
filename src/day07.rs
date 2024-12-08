use std::fs;

pub fn day07() {
    let input = fs::read_to_string("day07_input.txt").unwrap();
    let sum = part1::part1(&input);
    println!("total calibration result: {}", sum);
}

type Num = u64;
type Out1 = Num;
type Out2 = ();
type Test = (Num, Vec<Num>);

mod part1 {
    use std::collections::HashSet;

    use crate::day07::parse::parse;

    use super::{Num, Out1, Test};

    pub fn part1(input: &str) -> Out1 {
        let tests = parse(input);
        let sum: Num = tests.iter().filter(|t| good(t)).map(|t| t.0).sum();
        sum
    }

    pub fn good(test: &Test) -> bool {
        let (label, nums) = test;
        let num_set: HashSet<Num> = HashSet::from_iter(nums.iter().copied());
        if nums.iter().copied().product::<Num>() == *label
            || nums.iter().copied().sum::<Num>() == *label
        {
            true
        } else {
            for permutation in AllPerms::new(nums.as_slice()) {
                let perm_set = HashSet::from_iter(permutation.iter().copied());
                let sum: Num = permutation.iter().sum();
                if *label - sum == num_set.difference(&perm_set).copied().product() {
                    return true;
                }
            }
            false
        }
    }

    pub struct AllPerms<'a> {
        nums: &'a [Num],
        i: usize,
        cur_perm: Option<Vec<Vec<Num>>>,
    }

    impl<'a> AllPerms<'a> {
        pub fn new(nums: &'a [Num]) -> Self {
            Self {
                nums,
                i: 1,
                cur_perm: None,
            }
        }
    }

    impl Iterator for AllPerms<'_> {
        type Item = Vec<Num>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(p) = &mut self.cur_perm {
                let elem = p.pop().unwrap();
                if p.is_empty() {
                    self.cur_perm = None;
                }
                Some(elem)
            } else if self.i < self.nums.len() {
                let i = self.i;
                self.i += 1;
                let mut p_set: HashSet<Vec<Num>> = HashSet::new();
                for mut p in Permutations::new(self.nums.iter().copied()) {
                    p.drain(i..);
                    p_set.insert(p);
                }
                let mut p_vec = Vec::from_iter(p_set.drain());
                let elem = p_vec.pop().unwrap();
                if !p_vec.is_empty() {
                    self.cur_perm = Some(p_vec);
                }
                Some(elem)
            } else {
                None
            }
        }
    }

    pub struct Permutations {
        nums: Vec<Num>,
        end: bool,
    }

    impl Permutations {
        pub fn new<I>(nums: I) -> Self
        where
            I: Iterator<Item = Num>,
        {
            Self {
                nums: Vec::from_iter(nums),
                end: false,
            }
        }

        fn find_largest<I, F>(&self, indices: I, f: F) -> Option<usize>
        where
            F: Fn(usize) -> bool,
            I: Iterator<Item = usize>,
        {
            indices.filter(|i| f(*i)).last()
        }
    }

    impl Iterator for Permutations {
        type Item = Vec<Num>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                None
            } else {
                let out = self.nums.clone();
                let len = self.nums.len();
                if let Some(k) =
                    self.find_largest(0..(len - 1), |k| self.nums[k] < self.nums[k + 1])
                {
                    let l = self
                        .find_largest(k..len, |l| self.nums[k] < self.nums[l])
                        .unwrap();
                    self.nums.swap(k, l);
                    self.nums[(k + 1)..len].reverse();
                } else {
                    self.end = true;
                }
                Some(out)
            }
        }
    }
}

mod parse {
    //! tests: line+ trailer
    //! line: label nums '\n'
    //! label: num ':'
    //! nums: num num+
    //! trailer: '\n'*
    use std::iter::Peekable;

    use token::{Token, Tokenizer};

    use super::{Num, Test};

    pub fn parse(input: &str) -> Vec<Test> {
        let mut t: Peekable<Tokenizer<'_>> = Tokenizer::new(input).peekable();
        let mut tests = vec![line(&mut t)];
        while let Some(&Token::Num(_)) = t.peek() {
            tests.push(line(&mut t));
        }
        trailer(&mut t);
        assert!(t.peek().is_none());
        tests
    }

    fn trailer(t: &mut Peekable<Tokenizer<'_>>) {
        while let Some(&Token::NewLine) = t.peek() {
            t.next().unwrap();
        }
    }

    fn line(t: &mut Peekable<Tokenizer<'_>>) -> (Num, Vec<Num>) {
        let label = label(t);
        let nums = nums(t);
        assert_eq!(t.next(), Some(Token::NewLine));
        (label, nums)
    }

    fn label(t: &mut Peekable<Tokenizer<'_>>) -> Num {
        let num = t.next().unwrap().num();
        assert_eq!(t.next(), Some(Token::Colon));
        num
    }

    fn nums(t: &mut Peekable<Tokenizer<'_>>) -> Vec<Num> {
        let mut nums: Vec<Num> = Vec::with_capacity(2);
        nums.push(t.next().unwrap().num());
        nums.push(t.next().unwrap().num());

        while let Some(Token::Num(num)) = t.peek() {
            nums.push(*num);
            t.next().unwrap();
        }
        nums
    }

    mod token {
        use std::{iter::Peekable, str::Chars};

        use crate::day07::Num;

        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum Token {
            Num(Num),
            Colon,
            NewLine,
        }

        impl Token {
            pub fn num(self) -> Num {
                if let Self::Num(i) = self {
                    i
                } else {
                    panic!("expected number")
                }
            }
        }

        pub struct Tokenizer<'a> {
            chars: Peekable<Chars<'a>>,
        }

        impl<'a> Tokenizer<'a> {
            pub fn new(s: &'a str) -> Self {
                Self {
                    chars: s.chars().peekable(),
                }
            }

            fn advance(&mut self) {
                self.chars.next().unwrap();
            }

            fn num(&mut self) -> Num {
                let mut i = self.chars.next().unwrap().to_digit(10).unwrap() as Num;
                while let Some(c) = self.chars.peek() {
                    if let Some(digit) = c.to_digit(10) {
                        self.advance();
                        i = i * 10 + (digit as Num);
                    } else {
                        break;
                    }
                }
                i
            }
        }

        impl Iterator for Tokenizer<'_> {
            type Item = Token;

            fn next(&mut self) -> Option<Self::Item> {
                while let Some(c) = self.chars.peek() {
                    if *c == '\n' {
                        self.advance();
                        return Some(Token::NewLine);
                    } else if *c == ':' {
                        self.advance();
                        return Some(Token::Colon);
                    } else if c.is_ascii_digit() {
                        return Some(Token::Num(self.num()));
                    } else if c.is_whitespace() {
                        self.advance();
                    } else {
                        panic!("unexpected character '{}'", *c);
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day07::{
        parse::parse,
        part1::{self, good},
        Test,
    };

    use super::{
        part1::{AllPerms, Permutations},
        Num,
    };

    #[test]
    fn test_all_perms() {
        let i: [Num; 3] = [1, 2, 3];
        let j: HashSet<Vec<Num>> = HashSet::from([
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![2, 1],
            vec![1, 3],
            vec![3, 1],
            vec![2, 3],
            vec![3, 2],
        ]);
        let k: HashSet<Vec<Num>> = HashSet::from_iter(AllPerms::new(&i));
        assert_eq!(j, k);
    }

    #[test]
    fn test_parse() {
        let input = "123: 4 56 78
11: 12 1 4 12
";
        assert_eq!(
            parse(input),
            vec![(123, vec![4, 56, 78]), (11, vec!(12, 1, 4, 12))]
        );
    }

    #[test]
    fn test_good() {
        let tests: [Test; 3] = [
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (292, vec![11, 6, 16, 20]),
        ];
        for test in &tests {
            assert!(good(test));
        }
    }

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!(part1::part1(input), 3749);
    }

    fn fact(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            (1..=n).product()
        }
    }

    #[test]
    fn test_perm() {
        let i: [Num; 3] = [1, 2, 3];
        let j = HashSet::from([
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
        assert_eq!(j.len(), fact(i.len()));
        let k = Vec::from_iter(Permutations::new(i.iter().copied()));
        assert_eq!(j.len(), k.len());
        assert_eq!(HashSet::from_iter(k.iter().cloned()), j);
    }
}
