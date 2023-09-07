mod checkers;

fn main() {
    for i in checkers::CHECKERS {
        println!("{}", i.name());
    }
}
