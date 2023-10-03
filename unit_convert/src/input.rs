use std::{result, str::FromStr};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::Num;

static SEPARATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"=>|->|to").unwrap());

#[derive(Debug)]
pub struct Input {
    pub value: Num,
    pub from_unit: String,
    pub to_unit: String,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(inp: &str) -> result::Result<Self, Self::Err> {
        let mut parts = SEPARATOR.split(inp);
        let (from, to) = (
            parts.next().unwrap(),
            parts.next().context("No separator found.")?,
        );

        let (num, from) = pull_number(from.trim())?;

        Ok(Input {
            value: num,
            from_unit: from,
            to_unit: to.trim().to_owned(),
        })
    }
}

fn pull_number(raw: &str) -> Result<(Num, String)> {
    let mut num = String::new();

    let mut chars = raw.chars().peekable();
    while let Some('0'..='9' | '.' | '-') = chars.peek() {
        num.push(chars.next().unwrap());
    }

    let num = num.parse::<Num>()?;
    let remaining = chars.collect::<String>();

    Ok((num, remaining))
}
