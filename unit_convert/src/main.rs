use std::{env, str::FromStr};

use anyhow::Result;

use unit_convert::{dimension::Dimensions, input};

fn main() -> Result<()> {
    let inp = env::args().skip(1).collect::<String>();
    let inp = input::Input::from_str(&inp)?;

    let from_dim = Dimensions::from_str(&inp.from_unit)?;
    let to_dim = Dimensions::from_str(&inp.to_unit)?;
    println!(
        "| {}\n| {}\n\\== {}",
        from_dim.as_base_units(),
        to_dim.as_base_units(),
        from_dim == to_dim
    );

    let val = from_dim.convert(&to_dim, inp.value)?;
    println!("{}{} => {}{}", inp.value, inp.from_unit, val, inp.to_unit);
    Ok(())
}
