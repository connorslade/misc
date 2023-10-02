mod dimensional_analysis;
mod input;
mod prefix;
mod units;
mod dimension;

use std::str::FromStr;

type Num = f64;

fn main() {
    let inp = "10s => m";
    let inp = input::Input::from_str(inp);
    dbg!(inp);
}

// m/s^2
// dim => m{1}, s{-2}