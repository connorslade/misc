use std::str::FromStr;

#[derive(Clone, PartialEq, Eq)]
pub struct Card {
    pub question: String,
    pub answer: String,
    pub status: CardStatus,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CardStatus {
    New,
    Learning,
    Mastered,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '\t');
        let answer = parts.next().ok_or(())?.to_string();
        let question = parts.next().ok_or(())?.to_string();

        Ok(Self {
            question,
            answer,
            status: CardStatus::New,
        })
    }
}

impl ToString for CardStatus {
    fn to_string(&self) -> String {
        match self {
            CardStatus::New => "NEW",
            CardStatus::Learning => "LEARNING",
            CardStatus::Mastered => "MASTERED",
        }.to_string()
    }
}