use std::{fs, io, path::Path, str::FromStr};

use rand::{seq::SliceRandom, Rng};

use crate::{
    card::{Card, CardStatus},
    CARDS_PER_SECTION,
};

pub struct Set {
    pub name: String,
    pub cards: Vec<Card>,

    pub sections: Vec<Vec<Card>>,
    pub current_section: usize,
    pub current_card: usize,
}

impl Set {
    pub fn load(file: impl AsRef<Path>) -> io::Result<Self> {
        let raw = fs::read_to_string(file)?;
        let mut cards = Vec::new();
        let mut success = true;

        for line in raw.lines() {
            let card = match Card::from_str(line) {
                Ok(card) => card,
                Err(_) => {
                    eprintln!("Invalid card: {}", line);
                    success = false;
                    continue;
                }
            };
            cards.push(card);
        }

        if !success {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid card(s)",
            ));
        }
        println!("Loaded {} cards", cards.len());

        let mut sections = Vec::new();
        for i in 0..(cards.len() / CARDS_PER_SECTION) {
            let start = i * CARDS_PER_SECTION;
            let end = start + CARDS_PER_SECTION;
            sections.push(cards[start..end].to_vec());
        }

        Ok(Self {
            name: "Default".to_string(),
            cards,
            sections,
            current_section: 0,
            current_card: 0,
        })
    }

    pub fn get_current_card(&self) -> &Card {
        &self.sections[self.current_section][self.current_card]
    }

    // Pick four random cards from the current section, making sure not to duplicate any answers
    pub fn get_options(&self) -> Vec<String> {
        let mut options = Vec::new();
        let current_card = self.get_current_card();

        let mut rng = rand::thread_rng();
        while options.len() < 4 {
            let card = self.cards.choose(&mut rng).unwrap();
            if !options.contains(&card.answer) {
                options.push(card.answer.clone());
            }
        }

        if !options.contains(&current_card.answer) {
            options[rng.gen_range(0..4)] = current_card.answer.clone();
        }

        options
    }

    pub fn answer(&mut self, ans: &str) -> bool {
        let mut current_card = &mut self.sections[self.current_section][self.current_card];

        if ans == current_card.answer {
            current_card.status = CardStatus::Mastered;
            self.current_card += 1;
            if self.current_card >= CARDS_PER_SECTION {
                self.current_card = 0;
                self.current_section += 1;
                if self.current_section >= self.sections.len() {
                    println!("Finished!");
                    std::process::exit(0);
                }
            }
            return true;
        }

        current_card.status = CardStatus::Learning;
        false
    }
}
