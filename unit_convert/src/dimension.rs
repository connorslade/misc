use std::str::FromStr;

use crate::{
    units::{Conversion, Space},
    Num,
};

pub struct Dimensions {
    /// Assumed to always be simplified, no two dimensions with the same space
    dimensions: Vec<Dimension>,
    units: Vec<&'static dyn Conversion>,
}

#[derive(Debug, PartialEq)]
pub struct Dimension {
    unit_space: Space,
    exponent: Num,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unit(String), // Box<dyn Conversion>
    Num(Num),
    Op(Op),
    Group(Vec<Token>),
    Tree(Op, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Op {
    #[default]
    Mul,
    Div,
    Pow,
}

impl Dimensions {
    fn get_space(&self, space: Space) -> Option<Num> {
        self.dimensions
            .iter()
            .find(|x| x.unit_space == space)
            .map(|x| x.exponent)
    }
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Self::Mul => 2,
            Self::Div => 2,
            Self::Pow => 3,
        }
    }
}

impl FromStr for Dimension {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        if self.dimensions.len() != other.dimensions.len() {
            return false;
        }

        for dim in &self.dimensions {
            match other.get_space(dim.unit_space) {
                Some(i) if i == dim.exponent => {}
                _ => return false,
            }
        }

        true
    }
}

mod tree {
    use std::collections::HashMap;

    use super::Token;

    pub struct Treeifyer {
        tokens: Vec<Token>,
        precedence: HashMap<u8, u32>,
    }

    impl Treeifyer {
        pub fn treeify(mut tokens: Vec<Token>) -> Token {
            if tokens.len() == 1 {
                let token = tokens.pop().unwrap();
                match token {
                    Token::Group(tokens) => return Treeifyer::treeify(tokens),
                    Token::Op(..) | Token::Tree(..) => panic!("Invalid token"),
                    _ => return token,
                }
            }

            let mut ctx = Self::new(tokens);
            ctx.update_precedence_counts();
            ctx._treeify();

            assert_eq!(ctx.tokens.len(), 1);
            ctx.tokens.pop().unwrap()
        }

        fn new(tokens: Vec<Token>) -> Self {
            Self {
                tokens,
                precedence: HashMap::new(),
            }
        }

        // this is probably inefficient or something but my brain cant handle thinking about math for any longer
        fn _treeify(&mut self) {
            while self.tokens.len() > 1 {
                let mut i = 0;
                while i < self.tokens.len() {
                    let Token::Op(op) = self.tokens[i] else {
                        i += 1;
                        continue;
                    };

                    let max_precedence = self
                        .precedence
                        .iter()
                        .filter(|x| *x.1 > 0)
                        .max_by_key(|x| x.0)
                        .unwrap()
                        .0;
                    if op.precedence() < *max_precedence {
                        i += 1;
                        continue;
                    }

                    let left = self.tokens.remove(i - 1);
                    let right = self.tokens.remove(i);

                    self.tokens[i - 1] = Token::Tree(op.clone(), Box::new(left), Box::new(right));
                    self.precedence
                        .entry(op.precedence())
                        .and_modify(|x| *x -= 1);
                    break;
                }
            }
        }

        fn update_precedence_counts(&mut self) {
            for i in &self.tokens {
                if let Token::Op(op) = i {
                    *self.precedence.entry(op.precedence()).or_insert(0) += 1;
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::super::{Op, Token};
        use super::Treeifyer;

        #[test]
        fn test_tree() {
            let tokens = vec![
                Token::Unit("m".into()),
                Token::Op(Op::Div),
                Token::Unit("s".into()),
                Token::Op(Op::Pow),
                Token::Num(2.0),
            ];

            let tree = Treeifyer::treeify(tokens);

            assert_eq!(
                tree,
                Token::Tree(
                    Op::Div,
                    Box::new(Token::Unit("m".into())),
                    Box::new(Token::Tree(
                        Op::Pow,
                        Box::new(Token::Unit("s".into())),
                        Box::new(Token::Num(2.0)),
                    ))
                )
            );
        }
    }
}

mod tokenizer {
    use crate::Num;

    use super::{Op, Token};

    pub struct Tokenizer {
        chars: Box<[char]>,
        index: usize,
        depth: usize,

        pub tokens: Vec<Token>,
        buffer: String,
    }

    impl Tokenizer {
        pub fn tokenize(raw: &str) -> Vec<Token> {
            let mut ctx = Self::new(raw);

            while ctx.index < ctx.chars.len() {
                let chr = ctx.chars[ctx.index];
                ctx.index += 1;

                if ctx.depth > 0 {
                    match chr {
                        '(' => ctx.depth += 1,
                        ')' => {
                            ctx.depth -= 1;
                            if ctx.depth == 0 {
                                ctx.tokens
                                    .push(Token::Group(Tokenizer::tokenize(&ctx.buffer)));
                                ctx.buffer.clear();
                            }
                        }
                        _ => ctx.buffer.push(chr),
                    }
                    continue;
                }

                match chr {
                    x if x.is_whitespace() => continue,
                    '/' => ctx.add_token(Op::Div),
                    '*' => ctx.add_token(Op::Mul),
                    '^' => ctx.add_token(Op::Pow),
                    '(' => ctx.depth += 1,
                    ')' => panic!("Unmatched closing parenthesis"),
                    _ => ctx.buffer.push(chr),
                }
            }

            ctx.flush_buffer();
            ctx.tokens
        }

        fn new(input: &str) -> Self {
            Self {
                chars: input.chars().collect(),
                index: 0,
                depth: 0,

                tokens: Vec::new(),
                buffer: String::new(),
            }
        }

        fn take_int(&mut self) -> i8 {
            let mut number = String::new();
            while self.index < self.chars.len() {
                let chr = self.chars[self.index];
                if chr.is_ascii_digit() || chr == '-' {
                    number.push(chr);
                } else {
                    break;
                }

                self.index += 1;
            }

            number.parse().unwrap()
        }

        fn add_token(&mut self, op: Op) {
            self.flush_buffer();
            self.tokens.push(Token::Op(op));
        }

        fn flush_buffer(&mut self) {
            if self.buffer.is_empty() {
                return;
            }

            if let Ok(num) = self.buffer.parse::<Num>() {
                self.tokens.push(Token::Num(num));
            } else {
                self.tokens.push(Token::Unit(self.buffer.clone()));
            }

            self.buffer.clear();
        }
    }

    #[cfg(test)]
    mod test {
        use super::{Op, Token, Tokenizer};

        #[test]
        fn test_tokenize() {
            let tokens = Tokenizer::tokenize("m/s^2");
            assert_eq!(
                tokens,
                vec![
                    Token::Unit("m".into()),
                    Token::Op(Op::Div),
                    Token::Unit("s".into()),
                    Token::Op(Op::Pow),
                    Token::Num(2.0),
                ]
            );
        }

        #[test]
        fn test_tokenize_2() {
            let tokens = Tokenizer::tokenize("m / (s * s)");
            assert_eq!(
                tokens,
                vec![
                    Token::Unit("m".into()),
                    Token::Op(Op::Div),
                    Token::Group(vec![
                        Token::Unit("s".into()),
                        Token::Op(Op::Mul),
                        Token::Unit("s".into()),
                    ]),
                ]
            );
        }
    }
}
