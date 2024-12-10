use crate::types::Type;

pub struct CaseParser {
    chars: Vec<char>,
    idx: usize,
}

impl CaseParser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
        }
    }

    pub fn parse(&mut self) -> (Vec<Type>, Type) {
        let mut args = Vec::new();
        while let Some(next) = self.next_type() {
            args.push(next);
            self.take_separator();
        }

        let result = args.pop().unwrap();

        (args, result)
    }

    fn eof(&self) -> bool {
        self.idx >= self.chars.len()
    }

    fn next_type(&mut self) -> Option<Type> {
        let next = *self.chars.get(self.idx)?;
        if num_char(next) {
            return self.take_number().map(|x| Type::Number(x));
        } else if matches!(next, '"' | '\'') {
            return self.take_string().map(|x| Type::String(x));
        } else if next.to_ascii_lowercase() == 't' {
            self.idx += 4;
            return Some(Type::Bool(true));
        } else if next.to_ascii_lowercase() == 'f' {
            self.idx += 5;
            return Some(Type::Bool(false));
        } else if next == '[' {
            return self.take_array().map(|x| Type::ArrayList(x));
        } else if next == '{' {
            return self.take_array().map(|x| Type::Array(x));
        }

        None
    }

    fn take_separator(&mut self) {
        while !self.eof() && matches!(self.chars[self.idx], ' ' | ',') {
            self.idx += 1;
        }
    }

    fn take_number(&mut self) -> Option<f32> {
        let mut working = String::new();
        while !self.eof() && num_char(self.chars[self.idx]) {
            working.push(self.chars[self.idx]);
            self.idx += 1;
        }

        (!working.is_empty())
            .then(|| working.parse::<f32>().ok())
            .flatten()
    }

    fn take_string(&mut self) -> Option<String> {
        let first = self.chars.get(self.idx)?;
        if !matches!(first, '"' | '\'') {
            return None;
        }

        let start = self.idx + 1;
        let mut end = start;
        while self.chars.get(end)? != first {
            end += 1;
        }

        self.idx += end - start + 2;
        Some(self.chars[start..end].iter().collect())
    }

    fn take_array(&mut self) -> Option<Vec<Type>> {
        let first = self.chars.get(self.idx)?;
        self.idx += 1;
        if !matches!(first, '[' | '{') {
            return None;
        }

        let mut out = Vec::new();

        loop {
            self.take_separator();
            if matches!(self.chars[self.idx], ']' | '}') {
                self.idx += 1;
                break;
            };
            out.push(self.next_type()?);
        }

        Some(out)
    }
}

fn num_char(chr: char) -> bool {
    matches!(chr, '0'..='9' | '.' | '-')
}
