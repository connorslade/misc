use std::{collections::HashMap, str::FromStr};

use crate::{
    dimension::{expander::Expander, tokenizer::Tokenizer, tree::Treeifyer},
    units::{Conversion, Space},
    Num,
};

#[derive(Debug)]
pub struct Dimensions {
    /// Assumed to always be simplified, no two dimensions with the same space
    dimensions: HashMap<Space, Num>,
    units: Vec<Unit>,
}

#[derive(Debug)]
struct Unit {
    conversion: &'static dyn Conversion,
    power: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Unit {
        conversion: &'static &'static dyn Conversion,
        power: i32,
    },
    Num(Num),
    Op(Op),
    Group(Vec<Token>),
    Tree(Op, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Op {
    #[default]
    Mul,
    Div,
    Pow,
}

impl Dimensions {
    pub fn convert(mut self, mut other: Dimensions, mut value: Num) -> Num {
        for i in &self.units {
            if self.dimensions.get(&i.conversion.space()).unwrap().signum() == 0.0 {
                value = i.conversion.from_base(&value);
            } else {
                value = i.conversion.to_base(&value);
            }
            value *= (10.0 as Num).powf(i.power as Num);
            self.dimensions
                .entry(i.conversion.space())
                .and_modify(|x| *x -= i.power as Num);
        }

        dbg!(value);
        for i in &other.units {
            if other
                .dimensions
                .get(&i.conversion.space())
                .unwrap()
                .signum()
                == 0.0
            {
                value = i.conversion.from_base(&value);
            } else {
                value = i.conversion.to_base(&value);
            }
            value *= (10.0 as Num).powf(i.power as Num);
            other
                .dimensions
                .entry(i.conversion.space())
                .and_modify(|x| *x -= i.power as Num);
        }

        // for i in [self, other] {
        //     if i.dimensions
        //         .iter()
        //         .filter(|(_, &count)| count > 0.0)
        //         .count()
        //         != 0
        //     {
        //         panic!("Invalid dimensions");
        //     }
        // }

        value
    }

    fn get_space(&self, space: Space) -> Option<Num> {
        self.dimensions
            .iter()
            .find(|(&unit_space, _)| unit_space == space)
            .map(|(_, &exponent)| exponent)
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

impl FromStr for Dimensions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Tokenizer::tokenize(s);
        let tree = Treeifyer::treeify(tokens);
        let (dimensions, units) = Expander::expand(tree);

        Ok(Dimensions { dimensions, units })
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        if self.dimensions.len() != other.dimensions.len() {
            return false;
        }

        for (&space, &exponent) in &self.dimensions {
            match other.get_space(space) {
                Some(i) if i == exponent => {}
                _ => return false,
            }
        }

        true
    }
}

mod expander {
    use std::collections::HashMap;

    use crate::{units::Space, Num};

    use super::{Op, Token, Unit};

    pub struct Expander {
        dimensions: HashMap<Space, Num>,
        units: Vec<Unit>,
    }

    impl Expander {
        pub(super) fn expand(token: Token) -> (HashMap<Space, f64>, Vec<Unit>) {
            let mut exp = Self::new();
            exp._expand(token, 1.0);

            (exp.dimensions, exp.units)
        }

        fn new() -> Self {
            Self {
                dimensions: HashMap::new(),
                units: Vec::new(),
            }
        }

        fn _expand(&mut self, token: Token, exponent: Num) {
            match token {
                Token::Tree(op, left, right) => match op {
                    Op::Pow => {
                        self._expand(
                            *left,
                            exponent
                                * match *right {
                                    Token::Num(num) => num,
                                    _ => panic!("Invalid exponent. (Expected number)"),
                                },
                        );
                    }
                    Op::Div => {
                        self._expand(*left, exponent);
                        self._expand(*right, -exponent);
                    }
                    _ => {
                        self._expand(*left, exponent);
                        self._expand(*right, exponent);
                    }
                },
                Token::Unit { conversion, power } => {
                    self.add_dimension(conversion.space(), exponent);
                    self.units.push(Unit {
                        conversion: *conversion,
                        power: if power == 1 { 0 } else { power },
                    });
                }
                Token::Group(group) => {
                    for i in group {
                        self._expand(i, exponent);
                    }
                }
                Token::Num(..) | Token::Op(..) => unreachable!(),
            }
        }

        fn add_dimension(&mut self, space: Space, exp: Num) {
            self.dimensions
                .entry(space)
                .and_modify(|x| *x += exp)
                .or_insert(exp);
        }
    }

    #[cfg(test)]
    mod test {
        use crate::{
            dimension::{Op, Token},
            units::{duration::Second, length::Meter, Conversion},
        };

        use super::Expander;

        #[test]
        fn test_expander() {
            let sec = &(&Second as &'static dyn Conversion);
            let meter = &(&Meter as &'static dyn Conversion);

            let inp = Token::Tree(
                Op::Div,
                Box::new(Token::Unit {
                    conversion: meter,
                    power: 1,
                }),
                Box::new(Token::Tree(
                    Op::Pow,
                    Box::new(Token::Unit {
                        conversion: sec,
                        power: 1,
                    }),
                    Box::new(Token::Num(2.0)),
                )),
            );

            let (exp, _) = Expander::expand(inp);
            assert_eq!(exp.len(), 2);
            assert_eq!(exp.get(&meter.space()), Some(&1.0));
            assert_eq!(exp.get(&sec.space()), Some(&-2.0));
        }
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
        pub(super) fn treeify(mut tokens: Vec<Token>) -> Token {
            if tokens.len() == 1 {
                let token = tokens.pop().unwrap();
                match token {
                    Token::Group(tokens) => return Treeifyer::treeify(tokens),
                    Token::Unit { .. } => return token,
                    Token::Op(..) | Token::Tree(..) | Token::Num(..) => panic!("Invalid token"),
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

                    let make_tree = |x| match x {
                        Token::Group(tokens) => Treeifyer::treeify(tokens),
                        _ => x,
                    };

                    let left = make_tree(self.tokens.remove(i - 1));
                    let right = make_tree(self.tokens.remove(i));

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
        use crate::units::{
            duration::{Minute, Second},
            Conversion,
        };

        #[test]
        fn test_tree() {
            let sec = &(&Second as &'static dyn Conversion);
            let min = &(&Minute as &'static dyn Conversion);

            let tokens = vec![
                Token::Unit {
                    conversion: min,
                    power: 1,
                },
                Token::Op(Op::Div),
                Token::Unit {
                    conversion: sec,
                    power: 1,
                },
                Token::Op(Op::Pow),
                Token::Num(2.0),
            ];

            let tree = Treeifyer::treeify(tokens);

            assert_eq!(
                tree,
                Token::Tree(
                    Op::Div,
                    Box::new(Token::Unit {
                        conversion: min,
                        power: 1,
                    },),
                    Box::new(Token::Tree(
                        Op::Pow,
                        Box::new(Token::Unit {
                            conversion: sec,
                            power: 1,
                        },),
                        Box::new(Token::Num(2.0)),
                    ))
                )
            );
        }
    }
}

mod tokenizer {
    use crate::{prefix, Num};

    use super::{Op, Token};

    pub struct Tokenizer {
        chars: Box<[char]>,
        index: usize,
        depth: usize,

        pub(super) tokens: Vec<Token>,
        buffer: String,
    }

    impl Tokenizer {
        pub(super) fn tokenize(raw: &str) -> Vec<Token> {
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
            } else if let Some((conversion, power)) = prefix::get(&self.buffer) {
                self.tokens.push(Token::Unit {
                    conversion: conversion,
                    power: power.map(|x| x.power).unwrap_or(1),
                });
            } else {
                panic!("Invalid token: {}", self.buffer);
            }

            self.buffer.clear();
        }
    }

    #[cfg(test)]
    mod test {
        use crate::units::{duration::Second, length::Meter, Conversion};

        use super::{Op, Token, Tokenizer};

        #[test]
        fn test_tokenize() {
            let sec = &(&Second as &'static dyn Conversion);
            let meter = &(&Meter as &'static dyn Conversion);

            let tokens = Tokenizer::tokenize("m/s^2");
            assert_eq!(
                tokens,
                vec![
                    Token::Unit {
                        conversion: meter,
                        power: 1,
                    },
                    Token::Op(Op::Div),
                    Token::Unit {
                        conversion: sec,
                        power: 0,
                    },
                    Token::Op(Op::Pow),
                    Token::Num(2.0),
                ]
            );
        }

        #[test]
        fn test_tokenize_2() {
            let sec = &(&Second as &'static dyn Conversion);
            let meter = &(&Meter as &'static dyn Conversion);

            let tokens = Tokenizer::tokenize("m / (s * s)");
            assert_eq!(
                tokens,
                vec![
                    Token::Unit {
                        conversion: meter,
                        power: 1,
                    },
                    Token::Op(Op::Div),
                    Token::Group(vec![
                        Token::Unit {
                            conversion: sec,
                            power: 1,
                        },
                        Token::Op(Op::Mul),
                        Token::Unit {
                            conversion: sec,
                            power: 1,
                        }
                    ]),
                ]
            );
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Dimensions;

    #[test]
    fn test_dimensions() {
        let a = "m/s^2";
        let b = "m/(s*s)";
        let c = "m/s/s";

        let a = Dimensions::from_str(a).unwrap();
        for i in &[b, c] {
            let j = Dimensions::from_str(i).unwrap();
            assert_eq!(a, j, "Failed on: `{i}`");
        }
    }
}
