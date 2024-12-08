use std::fs;

use perms::OpPerms;

pub fn day07() {
    let input = fs::read_to_string("day07_input.txt").unwrap();
    let sum = part1::part1(&input);
    println!("total calibration result: {}", sum);
    let sum = part2::part2(&input);
    println!("total concatted calibration result: {}", sum);
}

type Num = u64;
type Out1 = Num;
type Out2 = Num;
type Test = (Num, Vec<Num>);

mod part1 {
    use crate::day07::parse::parse;

    use super::{good, Binary, Num, Operator, Out1};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Op {
        Add,
        Mul,
    }

    impl Operator for Op {
        const FIRST: Self = Self::Add;

        fn next(&self) -> Option<Self>
        where
            Self: Sized,
        {
            match self {
                Op::Add => Some(Self::Mul),
                Op::Mul => None,
            }
        }
    }

    impl Binary<Num> for Op {
        fn exec(&self, left: Num, right: Num) -> Num {
            match self {
                Op::Add => left + right,
                Op::Mul => left * right,
            }
        }
    }

    pub fn part1(input: &str) -> Out1 {
        let tests = parse(input);
        let sum: Out1 = tests
            .iter()
            .filter(|t| good::<Op, Num>(t))
            .map(|t| t.0)
            .sum();
        sum
    }
}

fn good<T, N>(test: &(N, Vec<N>)) -> bool
where
    T: Operator + Binary<N> + Clone + Sized,
    N: Sized + PartialEq + Clone,
{
    let (label, nums) = test;
    for ops in OpPerms::<T>::new(nums.len() - 1) {
        if exec(nums.as_slice(), &ops) == label.clone() {
            return true;
        }
    }
    false
}

mod part2 {
    use super::{good, parse::parse, Binary, Num, Operator, Out2};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Op {
        Add,
        Mul,
        Concat,
    }

    impl Operator for Op {
        const FIRST: Self = Self::Add;

        fn next(&self) -> Option<Self>
        where
            Self: Sized,
        {
            match self {
                Op::Add => Some(Self::Mul),
                Op::Mul => Some(Self::Concat),
                Op::Concat => None,
            }
        }
    }

    impl Binary<Num> for Op {
        fn exec(&self, left: Num, right: Num) -> Num {
            match self {
                Op::Add => left + right,
                Op::Mul => left * right,
                Op::Concat => concat_num(left, right),
            }
        }
    }

    pub fn concat_num(left: Num, right: Num) -> Num {
        left * (10 as Num).pow(ndigits(right) as u32) + right
    }

    fn ndigits(n: Num) -> u8 {
        if n == 0 {
            1
        } else {
            let mut n = n;
            let mut i = 0;
            while n != 0 {
                i += 1;
                n /= 10;
            }
            i
        }
    }

    pub fn part2(input: &str) -> Out2 {
        let tests = parse(input);
        let sum: Out2 = tests
            .iter()
            .filter(|t| good::<Op, Num>(t))
            .map(|t| t.0)
            .sum();
        sum
    }
}

trait Operator {
    const FIRST: Self;
    fn next(&self) -> Option<Self>
    where
        Self: Sized;

    fn add_zero_with_carry(&self, carry: bool) -> (Self, bool)
    where
        Self: Clone,
    {
        if carry {
            if let Some(new_op) = self.next() {
                (new_op, false)
            } else {
                (Self::FIRST.clone(), true)
            }
        } else {
            (self.clone(), false)
        }
    }
}

fn inc_ops<T>(ops: &[T]) -> (Vec<T>, bool)
where
    T: Operator + Sized + Clone,
{
    let mut new_ops = Vec::with_capacity(ops.len());
    let mut carry = true;
    for op in ops.iter().rev() {
        let (new_op, new_carry) = op.add_zero_with_carry(carry);
        new_ops.push(new_op);
        carry = new_carry;
    }
    new_ops.reverse();
    (new_ops, carry)
}

trait Binary<N> {
    fn exec(&self, left: N, right: N) -> N;
}

fn exec<T, N>(args: &[N], ops: &[T]) -> N
where
    T: Sized + Operator + Binary<N>,
    N: Clone,
{
    if args.len() == 2 {
        assert_eq!(ops.len(), 1);
        ops[0].exec(args[0].clone(), args[1].clone())
    } else {
        assert!(ops.len() > 1);
        let left = args[0].clone();
        let right = exec(&args[1..], &ops[1..]);
        ops[0].exec(left, right)
    }
}

mod perms {
    use std::mem;

    use super::{inc_ops, Operator};

    /// All permutations of a given number of operators.
    /// Since there's only 2 operators, we simulate binary incrementing.
    pub struct OpPerms<T> {
        ops: Vec<T>,
        end: bool,
    }

    impl<T> OpPerms<T>
    where
        T: Sized + Operator + Clone,
    {
        pub fn new(num_ops: usize) -> Self {
            assert!(num_ops > 0);
            Self {
                ops: vec![T::FIRST.clone(); num_ops],
                end: false,
            }
        }
    }

    impl<T> Iterator for OpPerms<T>
    where
        T: Sized + Operator + Clone,
    {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                None
            } else {
                let (new_ops, overflow) = inc_ops(&self.ops);
                if overflow {
                    self.end = true;
                }
                Some(mem::replace(&mut self.ops, new_ops))
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

    use crate::day07::{
        good,
        parse::parse,
        part1::{self, Op},
        part2::{self, concat_num},
        perms::OpPerms,
        Num, Test,
    };

    #[test]
    fn test_op1() {
        use part1::Op;
        let all = vec![
            vec![Op::Add, Op::Add],
            vec![Op::Add, Op::Mul],
            vec![Op::Mul, Op::Add],
            vec![Op::Mul, Op::Mul],
        ];
        assert_eq!(all, Vec::from_iter(OpPerms::<Op>::new(2)))
    }

    #[test]
    fn test_op2() {
        use part2::Op;
        let all = vec![
            vec![Op::Add, Op::Add],
            vec![Op::Add, Op::Mul],
            vec![Op::Add, Op::Concat],
            vec![Op::Mul, Op::Add],
            vec![Op::Mul, Op::Mul],
            vec![Op::Mul, Op::Concat],
            vec![Op::Concat, Op::Add],
            vec![Op::Concat, Op::Mul],
            vec![Op::Concat, Op::Concat],
        ];
        assert_eq!(all, Vec::from_iter(OpPerms::<Op>::new(2)));
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
            assert!(good::<Op, Num>(test));
        }
    }

    #[test]
    fn test_concat_good() {
        use part2::Op;

        assert!(good::<Op, Num>(&(156, vec![15, 6])));
    }

    #[test]
    fn test_examples() {
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
        assert_eq!(part2::part2(input), 11387);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat_num(123, 456), 123456);
        assert_eq!(concat_num(1, 0), 10);
        assert_eq!(concat_num(0, 123), 123);
    }
}
