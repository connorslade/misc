use std::io::{self, Stdout, Write};

use card::Card;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, SetTitle},
};
use getch::Getch;
use set::Set;

mod card;
mod set;

const CARDS_PER_SECTION: usize = 10;

fn main() {
    let mut set = Set::load("data.txt").unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, SetTitle("Flashcards")).unwrap();

    let mut waiting = false;
    let mut card = set.get_current_card().clone();
    let mut options = set.get_options();
    let getch = Getch::new();
    update(&set, &card, options.clone(), &mut stdout);
    loop {
        let event = getch.getch().unwrap() as char;
        if event == 'q' {
            break;
        }

        if waiting {
            waiting = false;
            update(&set, &card, options.clone(), &mut stdout);
            continue;
        }

        if event.is_numeric() {
            let index = event.to_digit(10).unwrap() as usize - 1;
            if index >= options.len() {
                continue;
            }

            let correct = set.answer(&options[index]);
            if !correct {
                waiting = true;
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    MoveTo(0, 0),
                    Print(format!(
                        "The correct answer was {}\n[ANY KEY TO CONTINUE]",
                        card.answer
                    ))
                )
                .unwrap();
                continue;
            }

            card = set.get_current_card().clone();
            options = set.get_options();
            update(&set, &card, options.clone(), &mut stdout);
        }
    }
}

fn update(set: &Set, card: &Card, options: Vec<String>, stdout: &mut Stdout) {
    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    queue!(
        stdout,
        Print(format!(
            "Section {}/{}\tCards {}/{}\t[Q]uit\n ({} left)\n\n",
            set.current_section + 1,
            set.sections.len(),
            set.current_card + 1,
            set.cards.len(),
            CARDS_PER_SECTION - set.current_card,
        ))
    )
    .unwrap();

    queue!(
        stdout,
        Print(format!(
            "> {}\n[{}]\n\n",
            card.question,
            card.status.to_string()
        ))
    )
    .unwrap();

    for (i, option) in options.iter().enumerate() {
        queue!(stdout, Print(format!("[{}] {}\n", i + 1, option))).unwrap();
    }

    stdout.flush().unwrap();
}
