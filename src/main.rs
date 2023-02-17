use std::io::{self, Stdout, Write};

use card::Card;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, SetTitle},
};
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
    update(&card, options.clone(), &mut stdout);
    loop {
        let event = read().unwrap();
        if waiting
            && matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    ..
                })
            )
        {
            waiting = false;
            update(&card, options.clone(), &mut stdout);
            continue;
        }

        if waiting {
            continue;
        }

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) if c.is_numeric() => {
                let index = c.to_digit(10).unwrap() as usize - 1;
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
                            "The correct answer was {}\n[SPACE TO CONTINUE]",
                            card.answer
                        ))
                    )
                    .unwrap();
                    continue;
                }

                card = set.get_current_card().clone();
                options = set.get_options();
                update(&card, options.clone(), &mut stdout);
            }
            _ => {}
        }
    }
}

fn update(card: &Card, options: Vec<String>, stdout: &mut Stdout) {
    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

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
