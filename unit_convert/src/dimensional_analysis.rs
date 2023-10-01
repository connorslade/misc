use std::default;

use crate::{
    units::{duration::Second, Conversion},
    Num,
};

struct Measurement {
    value: f64,
    unit: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Unit(String), // Box<dyn Conversion>
    Div,
    Mul,
    OpenParen,
    CloseParen,
    Tree(Op, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Op {
    #[default]
    Nop,
    Mul,
    Div,
}

impl Into<Token> for Op {
    fn into(self) -> Token {
        match self {
            Self::Nop => panic!("Cannot convert Nop into Token"),
            Self::Mul => Token::Mul,
            Self::Div => Token::Div,
        }
    }
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Self::Nop => 0,
            Self::Mul => 2,
            Self::Div => 2,
        }
    }
}

mod rpn {
    use super::{Op, Token};

    struct Rpn {
        idx: usize,
        tokens: Vec<Token>,
        output: Vec<Token>,
        stack: Vec<Op>,
    }

    impl Rpn {
        fn new(tokens: Vec<Token>) -> Self {
            Self {
                idx: 0,
                tokens,
                output: Vec::new(),
                stack: Vec::new(),
            }
        }

        fn rpn(&mut self) {
            while self.idx < self.tokens.len() {
                let i = &self.tokens[self.idx];
                self.idx += 1;

                match i {
                    Token::Div => self.handle_op(Op::Div),
                    Token::Mul => self.handle_op(Op::Mul),
                    Token::Unit(_) => self.output.push(i.clone()),
                    Token::OpenParen => todo!(),
                    Token::CloseParen => todo!(),
                    Token::Tree(_, _, _) => unreachable!(),
                }
            }

            while let Some(op) = self.stack.pop() {
                self.output.push(op.into());
            }
        }

        fn handle_op(&mut self, op: Op) {
            if self.stack.is_empty() {
                self.stack.push(op);
                return;
            }

            let precedence = op.precedence();
            while let Some(top) = self.stack.last() {
                if precedence < top.precedence() {
                    break;
                }

                self.output.push(self.stack.pop().unwrap().into());
            }
            self.stack.push(op.clone());
        }
    }

    #[cfg(test)]
    mod test {
        use super::Rpn;
        use crate::dimensional_analysis::Token;

        #[test]
        fn test_rpn() {
            let tokens = vec![
                Token::Unit("m".to_string()),
                Token::Div,
                Token::Unit("s".to_string()),
                Token::Div,
                Token::Unit("s".to_string()),
            ];

            let mut tree = Rpn::new(tokens);
            tree.rpn();

            assert_eq!(
                tree.output,
                vec![
                    Token::Unit("m".to_string()),
                    Token::Unit("s".to_string()),
                    Token::Div,
                    Token::Unit("s".to_string()),
                    Token::Div,
                ]
            );
        }

        fn test_rpn_2() {
            let tokens = vec![
                Token::Unit("m".to_string()),
                Token::Div,
                Token::OpenParen,
                Token::Unit("s".to_string()),
                Token::Mul,
                Token::Unit("s".to_string()),
                Token::CloseParen,
            ];

            let mut tree = Rpn::new(tokens);
            tree.rpn();

            assert_eq!(
                tree.output,
                vec![
                    Token::Unit("m".to_string()),
                    Token::Unit("s".to_string()),
                    Token::Unit("s".to_string()),
                    Token::Mul,
                    Token::Div,
                ]
            );
        }
    }
}

mod tokenizer {
    use super::Token;

    pub struct Tokenizer {
        chars: Vec<char>,
        index: usize,

        pub tokens: Vec<Token>,
        buffer: String,
    }

    impl Tokenizer {
        pub fn new(input: &str) -> Self {
            Self {
                chars: input.chars().collect(),
                index: 0,

                tokens: Vec::new(),
                buffer: String::new(),
            }
        }

        pub fn tokenize(&mut self) {
            while self.index < self.chars.len() {
                let chr = self.chars[self.index];
                self.index += 1;
                match chr {
                    '/' => self.add_token(Token::Div),
                    '*' => self.add_token(Token::Mul),
                    '(' => self.add_token(Token::OpenParen),
                    ')' => self.add_token(Token::CloseParen),
                    '^' => {
                        self.flush_buffer();
                        let exp = self.take_int();
                        let rep = self.tokens.pop().unwrap();

                        self.tokens.push(Token::OpenParen);
                        for _ in 1..exp {
                            self.tokens.push(rep.clone());
                            self.tokens.push(Token::Mul);
                        }
                        self.tokens.push(rep.clone());
                        self.tokens.push(Token::CloseParen);
                    }
                    _ => {
                        self.buffer.push(chr);
                    }
                }
            }

            self.flush_buffer();
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

        fn add_token(&mut self, token: Token) {
            self.flush_buffer();
            self.tokens.push(token);
        }

        fn flush_buffer(&mut self) {
            if self.buffer.is_empty() {
                return;
            }

            self.tokens.push(Token::Unit(self.buffer.clone()));
            self.buffer.clear();
        }
    }

    #[cfg(test)]
    mod test {
        use crate::dimensional_analysis::tokenizer::Tokenizer;

        #[test]
        fn test_tokenize() {
            let mut tokens = Tokenizer::new("m/s^2");
            tokens.tokenize();
            println!("{:?}", tokens.tokens);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dimensional_analysis::tokenizer::Tokenizer;

    use super::Token;

    #[test]
    fn test_dimensional_analysis() {
        let from = {
            let mut tokens = Tokenizer::new("m/h");
            tokens.tokenize();
            tokens.tokens
        };

        let to = {
            let mut tokens = Tokenizer::new("m/s");
            tokens.tokenize();
            tokens.tokens
        };

        let mut val = 1.0;
        for (a, b) in from.iter().zip(to.iter()) {
            match (a, b) {
                (Token::Unit(a), Token::Unit(b)) => {
                    println!("{} => {}", a, b);
                    let base = match a.as_str() {
                        "m" => val,
                        "s" => val,
                        "h" => val * 3600.0,
                        _ => panic!("Unknown unit"),
                    };
                    val = match b.as_str() {
                        "m" => base,
                        "s" => base,
                        "h" => base / 3600.0,
                        _ => panic!("Unknown unit"),
                    };
                }
                _ => {}
            }
        }

        println!("{}", val);
    }
}
