use std::fs;

pub fn day07() {
    let input = fs::read_to_string("day07_input.txt").unwrap();
    let sum = part1::part1(&input);
    println!("total calibration result: {}", sum);
}

type Num = u64;
type Out1 = Num;
type Out2 = Num;
type Test = (Num, Vec<Num>);

mod part1 {

    use crate::day07::parse::parse;

    use super::{exec, perms::OpPerms, Binary, Num, Operator, Out1, Test};

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
        let sum: Num = tests.iter().filter(|t| good(t)).map(|t| t.0).sum();
        sum
    }

    pub fn good(test: &Test) -> bool {
        let (label, nums) = test;
        for ops in OpPerms::<Op>::new(nums.len() - 1) {
            if exec(nums, &ops) == *label {
                return true;
            }
        }
        false
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
    let mut stack: Vec<N> = Vec::from_iter(args.iter().cloned().rev());
    for op in ops {
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(op.exec(left, right));
    }
    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
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
        parse::parse,
        part1::{self, good},
        perms::OpPerms,
        Test,
    };

    #[test]
    fn test_op_perms() {
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
}
