use std::default;

use crate::{
    units::{duration::Second, Conversion},
    Num,
};

struct Measurement {
    value: f64,
    // unit: Unit,
}

// struct Unit {
//     tree: Tree,
// }

/*
m/s^2
(m/s)/s

        UNIT
          |
         DIV
         /\
        s  s
        |
       DIV
       /\
      m  s
*/

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Unit(String), // Box<dyn Conversion>
    Div,
    Mul,
    Group(Vec<Token>),
    Tree(Op, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Op {
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

mod tree {
    use super::{Op, Token};

    struct Treeifier {
        idx: usize,
        tokens: Vec<Token>,
        output: Vec<Token>,
        stack: Vec<Op>,
    }

    // First convert into RPN
    // Then convert into tree

    impl Treeifier {
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
                    Token::Group(_) => todo!(),
                    Token::Tree(_, _, _) => unreachable!(),
                }
            }

            while let Some(op) = self.stack.pop() {
                self.output.push(op.into());
            }
        }

        fn tree(&mut self) {
            if self.output.len() == 1 {
                return;
            }

            self.idx = 0;
            while self.output.len() > 1 {
                let i = &self.output[self.idx];
                self.idx += 1;

                match i {
                    Token::Div => self.handle_tree(Op::Div),
                    Token::Mul => self.handle_tree(Op::Mul),
                    Token::Unit(_) => {},
                    _ => unreachable!(),
                }

                let left = self.output.remove(0);
                let right = self.output.remove(0);
                self.output
                    .push(Token::Tree(Op::Nop, Box::new(left), Box::new(right)));
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

        fn handle_tree(&mut self, op: Op) {
            if let Token::Tree(this_op, ..) = self.output.first_mut().unwrap() {
                this_op = op;
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::Treeifier;
        use crate::dimensional_analysis::Token;

        #[test]
        fn test_treeify() {
            let tokens = vec![
                Token::Unit("m".to_string()),
                Token::Div,
                Token::Unit("s".to_string()),
                Token::Div,
                Token::Unit("s".to_string()),
            ];

            let mut tree = Treeifier::new(tokens);
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
    }
}

mod tokenizer {
    use super::Token;

    struct Tokenizer {
        chars: Vec<char>,
        index: usize,

        tokens: Vec<Token>,
        buffer: String,
    }

    impl Tokenizer {
        fn new(input: &str) -> Self {
            Self {
                chars: input.chars().collect(),
                index: 0,

                tokens: Vec::new(),
                buffer: String::new(),
            }
        }

        fn tokenize(&mut self) {
            while self.index < self.chars.len() {
                let chr = self.chars[self.index];
                self.index += 1;
                match chr {
                    '/' => self.add_token(Token::Div),
                    '*' => self.add_token(Token::Mul),
                    '^' => {
                        self.flush_buffer();
                        let exp = self.take_int();
                        let rep = self.tokens.pop().unwrap();

                        let mut group = Vec::with_capacity(exp as usize * 2);
                        for _ in 1..exp {
                            group.push(rep.clone());
                            group.push(Token::Mul);
                        }
                        group.push(rep);
                        self.tokens.push(Token::Group(group));
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
