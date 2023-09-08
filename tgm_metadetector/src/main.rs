use std::{
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

mod checkers;

const INPUT: &str = "Eggs are a versatile and nutritious food enjoyed by people all over the world. They are a staple ingredient in many cuisines and are celebrated for their taste, versatility, and nutrient profile.
Eggs contain a variety of essential nutrients, including protein, vitamins, minerals, and healthy fats. They are an excellent source of high-quality protein, containing all the essential amino acids that our bodies need for growth, maintenance, and repair. Protein is essential for building and repairing tissues, supporting immune function, and producing enzymes and hormones.
Eggs are also a source of vitamins such as vitamin A, vitamin D, vitamin E, vitamin B12, and folate. These vitamins play crucial roles in various bodily functions, including promoting good vision, supporting bone health, protecting against oxidative damage, and maintaining a healthy nervous system.
In terms of minerals, eggs contain important nutrients like iron, selenium, and zinc. Iron is essential for carrying oxygen in the bloodstream, while selenium and zinc contribute to immune function and support overall health.
Contrary to earlier beliefs, studies have shown that moderate consumption of eggs does not lead to an increased risk of heart disease for most people. However, it's important to note that individuals with specific health conditions, such as diabetes or high cholesterol, may";

fn main() {
    let bar = MultiProgress::new();
    let barrier = Arc::new(Barrier::new(checkers::CHECKERS.len() + 1));

    for i in checkers::CHECKERS {
        let barrier = barrier.clone();
        let bar = bar.add(ProgressBar::new_spinner());
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message(i.name());

        thread::spawn(move || {
            match i.check(INPUT) {
                Ok(v) => {
                    bar.finish_with_message(format!(
                        "{}: {}% TGM",
                        i.name(),
                        (v * 1000.0).floor() / 10.0
                    ));
                }
                Err(e) => {
                    bar.finish_with_message(format!("{}: {}", i.name(), e));
                }
            }
            bar.finish();
            barrier.wait();
        });
    }

    barrier.wait();
}
