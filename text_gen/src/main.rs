use std::collections::HashMap;
use std::fs;

use rand::Rng;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace('\r', "");
    let tokens = tokenize(&input);

    let mut markov_chain = HashMap::new();
    let mut current = match tokens.first() {
        Some(token) => token.clone(),
        None => return,
    };

    for token in tokens.into_iter() {
        let entry = markov_chain.entry(current).or_insert_with(Vec::new);
        entry.push(token.clone());
        current = token;
    }

    dbg!(&markov_chain);

    let mut rng = rand::thread_rng();
    let mut current = match markov_chain.keys().next() {
        Some(token) => token.clone(),
        None => return,
    };
    loop {
        let next = match markov_chain.get(&current) {
            Some(ref entries) => &entries[rng.gen_range(0..entries.len())],
            None => break,
        };
        print!("{} ", next);
        current = next.clone();
    }

    println!();
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut word = String::new();

    for c in input.chars() {
        match c {
            ' ' | '\n' => {
                if !word.is_empty() {
                    tokens.push(word);
                    word = String::new();
                }
            }
            _ => word.push(c),
        }
    }

    if !word.is_empty() {
        tokens.push(word);
    }
    tokens
}
