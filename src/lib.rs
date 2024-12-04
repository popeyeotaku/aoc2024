pub mod day01 {
    use std::{collections::HashMap, fs, iter::zip};

    use parse::parse;

    pub fn day01() {
        println!("day 01");

        let input = fs::read_to_string("day01_input.txt").unwrap();
        let lines = parse(&input);
        let mut left = Vec::from_iter(lines.iter().map(|l| l.0));
        let mut right = Vec::from_iter(lines.iter().map(|l| l.1));
        left.sort();
        right.sort();

        let mut total_diff: u64 = 0;
        for (left, right) in zip(&left, &right) {
            total_diff += left.abs_diff(*right) as u64;
        }
        println!("total diff: {}", total_diff);

        let mut similarity: u128 = 0;

        let mut right_nums: HashMap<Num, u16> = HashMap::new();
        for i in &right {
            if let Some(count) = right_nums.get_mut(i) {
                *count += 1;
            } else {
                right_nums.insert(*i, 1);
            }
        }

        for i in &left {
            similarity += (*i as u128) * (*right_nums.get(i).unwrap_or(&0) as u128);
        }
        println!("similarity: {}", similarity);
    }

    type Num = u32;

    mod parse {
        //! pairs = line* ;
        //! line = num num '\n' ;

        use std::{iter::Peekable, str::Chars};

        use super::Num;

        pub fn parse(s: &str) -> Vec<(Num, Num)> {
            let mut tokenizer: Peekable<Tokenizer<'_>> = Tokenizer::new(s).peekable();
            let mut pairs: Vec<(Num, Num)> = Vec::new();

            while tokenizer.peek().is_some() {
                pairs.push(line(&mut tokenizer))
            }

            pairs
        }

        fn line(t: &mut Peekable<Tokenizer<'_>>) -> (Num, Num) {
            let left = t.next().unwrap().num();
            let right = t.next().unwrap().num();
            t.next().unwrap().newline();
            (left, right)
        }

        struct Tokenizer<'a> {
            chars: Peekable<Chars<'a>>,
        }

        impl<'a> Tokenizer<'a> {
            pub fn new(s: &'a str) -> Self {
                Self {
                    chars: s.chars().peekable(),
                }
            }

            fn num(&mut self) -> Num {
                let mut i: Num = self.chars.next().unwrap().to_digit(10).unwrap() as Num;
                while let Some(c) = self.chars.peek() {
                    if let Some(digit) = c.to_digit(10) {
                        self.chars.next().unwrap();
                        i = i * 10 + (digit as Num);
                    } else {
                        break;
                    }
                }
                i
            }
        }

        #[derive(Clone, Copy)]
        enum Token {
            NewLine,
            Num(Num),
        }

        impl Token {
            pub fn num(self) -> Num {
                match self {
                    Token::Num(i) => i,
                    _ => panic!("expected number"),
                }
            }

            pub fn newline(self) {
                match self {
                    Token::NewLine => (),
                    _ => panic!("expected newline"),
                }
            }
        }

        impl Iterator for Tokenizer<'_> {
            type Item = Token;

            fn next(&mut self) -> Option<Self::Item> {
                while let Some(c) = self.chars.peek() {
                    if *c == '\n' {
                        self.chars.next().unwrap();
                        return Some(Token::NewLine);
                    } else if c.is_numeric() {
                        return Some(Token::Num(self.num()));
                    } else if c.is_whitespace() {
                        self.chars.next().unwrap();
                    } else {
                        panic!("bad input character {c}");
                    }
                }
                None
            }
        }
    }
}

pub mod day02 {
    use std::fs;

    pub fn day02() {
        let input = fs::read_to_string("day02_input.txt").unwrap();
        let lines = parse::parse(&input);

        let mut safes: u32 = 0;
        for line in &lines {
            if analyze(line.as_slice()) {
                safes += 1;
            }
        }
        println!("safe reports: {}", safes);

        let mut safes: u32 = 0;
        for line in &lines {
            if find_safe(line.as_slice()) {
                safes += 1;
            }
        }
        println!("safes with toleration: {}", safes);
    }

    #[derive(Copy, Clone, PartialEq)]
    enum Direction {
        Inc,
        Dec,
        None,
    }

    impl Direction {
        pub fn dir(prev: Num, next: &Num) -> Self {
            match prev.cmp(next) {
                std::cmp::Ordering::Less => Self::Inc,
                std::cmp::Ordering::Greater => Self::Dec,
                std::cmp::Ordering::Equal => Self::None,
            }
        }
    }

    fn analyze(line: &[Num]) -> bool {
        let mut prev = line[0];
        let dir: Direction = Direction::dir(prev, &line[1]);
        if dir == Direction::None {
            return false;
        }
        for next in &line[1..] {
            if Direction::dir(prev, next) != dir {
                return false;
            }
            match prev.abs_diff(*next) {
                1..=3 => (),
                _ => return false,
            }
            prev = *next;
        }
        true
    }

    fn find_safe(line: &[Num]) -> bool {
        if analyze(line) {
            true
        } else {
            for i in 0..line.len() {
                let mut line = Vec::from_iter(line.iter().cloned());
                line.remove(i);
                if analyze(&line) {
                    return true;
                }
            }
            false
        }
    }

    type Num = u8;

    mod parse {
        use std::{iter::Peekable, str::Chars};

        use super::Num;

        enum Token {
            Num(Num),
            Newline,
        }

        impl Token {
            pub fn num(self) -> Num {
                if let Self::Num(i) = self {
                    i
                } else {
                    panic!("expected number")
                }
            }

            pub fn newline(self) {
                if let Self::Newline = self {
                } else {
                    panic!("expected newline")
                }
            }
        }

        struct Tokenizer<'a> {
            chars: Peekable<Chars<'a>>,
        }

        impl<'a> Tokenizer<'a> {
            pub fn new(s: &'a str) -> Self {
                Self {
                    chars: s.chars().peekable(),
                }
            }

            fn num(&mut self) -> Num {
                let mut i: Num = 0;

                while let Some(c) = self.chars.peek() {
                    if let Some(digit) = c.to_digit(10) {
                        self.chars.next().unwrap();
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
                        self.chars.next().unwrap();
                        return Some(Token::Newline);
                    } else if c.is_ascii_digit() {
                        return Some(Token::Num(self.num()));
                    } else if c.is_whitespace() {
                        self.chars.next().unwrap();
                    } else {
                        panic!("unexpected character {}", *c);
                    }
                }
                None
            }
        }

        fn line(t: &mut Peekable<Tokenizer<'_>>) -> Vec<Num> {
            let mut nums: Vec<Num> = vec![t.next().unwrap().num()];
            while let Some(Token::Num(i)) = t.peek() {
                nums.push(*i);
                t.next().unwrap();
            }
            t.next().unwrap().newline();
            nums
        }

        fn lines(t: &mut Peekable<Tokenizer<'_>>) -> Vec<Vec<Num>> {
            let mut lines: Vec<Vec<Num>> = vec![line(t)];
            while let Some(&Token::Num(_)) = t.peek() {
                lines.push(line(t));
            }
            lines
        }

        fn trailer(t: &mut Peekable<Tokenizer<'_>>) {
            for t in t.by_ref() {
                t.newline();
            }
        }

        pub fn parse(s: &str) -> Vec<Vec<Num>> {
            let mut t = Tokenizer::new(s).peekable();
            let l = lines(&mut t);
            trailer(&mut t);
            l
        }
    }
}

pub mod day03 {
    use std::fs;

    use op::interp;
    use parser::Parser;

    pub fn day03() {
        let input = fs::read_to_string("day03_input.txt").unwrap();
        let sum: u64 = Parser::new(&input)
            .mul_parser()
            .map(|(l, r)| (l as u64) * (r as u64))
            .sum();
        println!("sum of muls: {}", sum);
        let sum = interp(Parser::new(&input).op_parser());
        println!("sum of ops: {}", sum);
    }

    mod op {
        pub type Num = u16;

        #[derive(Clone, Copy)]
        pub enum Op {
            Mul(Num, Num),
            Do,
            Dont,
        }

        pub fn interp<I>(ops: I) -> u64
        where
            I: Iterator<Item = Op>,
        {
            let mut flag = true;
            let mut sum: u64 = 0;
            for op in ops {
                match op {
                    Op::Mul(left, right) => {
                        if flag {
                            sum += ((left as u32) * (right as u32)) as u64;
                        }
                    }
                    Op::Do => flag = true,
                    Op::Dont => flag = false,
                }
            }
            sum
        }
    }

    mod parser {
        //! program = mul*
        //! mul = "mul(" num ',' num ')
        //! num = digit digit? digit?

        use std::{iter::Peekable, str::Chars};

        use super::op::{Num, Op};

        pub struct Parser<'a> {
            chars: Peekable<Chars<'a>>,
        }

        enum WhichOp {
            Mul,
            Do,
            Dont,
        }

        impl<'a> Parser<'a> {
            pub fn new(s: &'a str) -> Self {
                Self {
                    chars: s.chars().peekable(),
                }
            }

            fn peek(&mut self) -> Option<char> {
                self.chars.peek().copied()
            }

            fn advance(&mut self) {
                self.chars.next();
            }

            fn eof(&mut self) -> bool {
                self.peek().is_none()
            }

            fn cmatch<F>(&mut self, f: F) -> Option<char>
            where
                F: FnOnce(char) -> bool,
            {
                if let Some(c) = self.peek() {
                    if f(c) {
                        self.advance();
                        return Some(c);
                    }
                }
                None
            }

            fn smatch(&mut self, s: &str) -> bool {
                for c in s.chars() {
                    if self.cmatch(|peeked| c == peeked).is_none() {
                        return false;
                    }
                }
                true
            }

            fn digit(&mut self) -> Option<u32> {
                self.cmatch(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap())
            }

            fn num(&mut self) -> Option<Num> {
                let mut i = self.digit()? as Num;
                for _ in 0..2 {
                    if let Some(digit) = self.digit() {
                        i = i * 10 + (digit as Num);
                    }
                }
                if self.digit().is_some() {
                    None
                } else {
                    Some(i)
                }
            }

            fn try_mul(&mut self) -> Option<(Num, Num)> {
                if self.smatch("mul(") {
                    let left = self.num()?;
                    self.cmatch(|c| c == ',')?;
                    let right = self.num()?;
                    self.cmatch(|c| c == ')')?;
                    Some((left, right))
                } else {
                    None
                }
            }

            fn which_op(&mut self) -> Option<WhichOp> {
                match self.peek()? {
                    'm' => {
                        if self.smatch("mul(") {
                            return Some(WhichOp::Mul);
                        }
                    }
                    'd' => {
                        if self.smatch("do") {
                            if self.cmatch(|c| c == '(').is_some() {
                                if self.cmatch(|c| c == ')').is_some() {
                                    return Some(WhichOp::Do);
                                }
                            } else if self.smatch("n't()") {
                                return Some(WhichOp::Dont);
                            }
                        }
                    }
                    _ => (),
                }
                None
            }

            pub fn mul_parser(self) -> MulParser<'a> {
                MulParser { p: self }
            }

            pub fn op_parser(self) -> OpParser<'a> {
                OpParser { p: self }
            }

            fn try_op(&mut self) -> Option<Op> {
                match self.which_op()? {
                    WhichOp::Mul => {
                        let left = self.num()?;
                        self.cmatch(|c| c == ',')?;
                        let right = self.num()?;
                        self.cmatch(|c| c == ')')?;
                        Some(Op::Mul(left, right))
                    }
                    WhichOp::Do => Some(Op::Do),
                    WhichOp::Dont => Some(Op::Dont),
                }
            }
        }

        pub struct MulParser<'a> {
            p: Parser<'a>,
        }

        impl Iterator for MulParser<'_> {
            type Item = (Num, Num);

            fn next(&mut self) -> Option<Self::Item> {
                while !self.p.eof() {
                    if let Some(pair) = self.p.try_mul() {
                        return Some(pair);
                    }
                    self.p.advance();
                }
                None
            }
        }

        pub struct OpParser<'a> {
            p: Parser<'a>,
        }

        impl Iterator for OpParser<'_> {
            type Item = Op;

            fn next(&mut self) -> Option<Self::Item> {
                while !self.p.eof() {
                    if let Some(op) = self.p.try_op() {
                        return Some(op);
                    }
                    self.p.advance();
                }
                None
            }
        }
    }
}

pub mod day04;
