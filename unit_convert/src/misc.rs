use std::borrow::Cow;

use crate::Num;

pub const SUPERSCRIPT_CHARSET: [char; 12] =
    ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', ' ', '⁻'];

pub trait NumToStringWithChars {
    fn to_string_with_chars(&self, chars: [char; 12]) -> Cow<'_, str>;
}

impl NumToStringWithChars for Num {
    fn to_string_with_chars(&self, chars: [char; 12]) -> Cow<'_, str> {
        if self.is_nan() {
            return Cow::Borrowed("NaN");
        }

        if self.is_infinite() {
            return Cow::Borrowed("inf");
        }

        // its bad i know
        // but i dont have time for this
        let out = self
            .to_string()
            .as_bytes()
            .iter()
            .map(|&c| {
                match c {
                    b'.' => return chars[10],
                    b'-' => return chars[11],
                    _ => {}
                }

                let c = c - b'0';
                if c < 10 {
                    return chars[c as usize];
                }

                panic!("invalid char: {}", c);
            })
            .collect::<String>();

        Cow::Owned(out)
    }
}
