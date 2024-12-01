pub mod day01 {
    use std::{fs, iter::zip};

    use parse::parse;

    pub fn day01() {
        let input = fs::read_to_string("day01_input.txt").unwrap();
        let lines = parse(&input);
        let mut left = Vec::from_iter(lines.iter().map(|l| l.0));
        let mut right = Vec::from_iter(lines.iter().map(|l| l.1));
        left.sort();
        right.sort();

        let mut sum: u64 = 0;
        for (left, right) in zip(left, right) {
            sum += left.abs_diff(right) as u64;
        }

        println!("{sum}");
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

pub fn day02() {
    todo!()
}
