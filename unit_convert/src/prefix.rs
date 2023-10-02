use crate::units::{find_unit, Conversion, UNIT_SPACES};

#[rustfmt::skip]
const METRIC_PREFIX: [Prefix; 24] = [
    Prefix::new("quetta", "Q",  30),
    Prefix::new("ronna",  "R",  27),
    Prefix::new("yotta",  "Y",  24),
    Prefix::new("zetta",  "Z",  21),
    Prefix::new("exa",    "E",  18),
    Prefix::new("peta",   "P",  15),
    Prefix::new("tera",   "T",  12),
    Prefix::new("giga",   "G",  9),
    Prefix::new("mega",   "M",  6),
    Prefix::new("kilo",   "k",  3),
    Prefix::new("hecto",  "h",  2),
    Prefix::new("deca",   "da", 1),
    Prefix::new("deci",   "d", -1),
    Prefix::new("centi",  "c", -2),
    Prefix::new("milli",  "m", -3),
    Prefix::new("micro",  "μ", -6),
    Prefix::new("nano",   "n", -9),
    Prefix::new("pico",   "p", -12),
    Prefix::new("femto",  "f", -15),
    Prefix::new("atto",   "a", -18),
    Prefix::new("zepto",  "z", -21),
    Prefix::new("yocto",  "y", -24),
    Prefix::new("ronto",  "r", -27),
    Prefix::new("quecto", "q", -30),
];

#[derive(Debug)]
pub struct Prefix {
    pub name: &'static str,
    pub symbol: &'static str,
    pub power: i32,
}

impl Prefix {
    const fn new(name: &'static str, symbol: &'static str, power: i32) -> Self {
        Self {
            name,
            symbol,
            power,
        }
    }
}

fn strip_prefix(s: &str) -> Option<(&str, &Prefix)> {
    for prefix in METRIC_PREFIX.iter() {
        for i in [prefix.name, prefix.symbol] {
            if s.starts_with(i) {
                return Some((&s[i.len()..], prefix));
            }
        }
    }

    None
}

fn get(s: &str) -> Option<(&&dyn Conversion, Option<&Prefix>)> {
    if let Some(i) = find_unit(s) {
        return Some((i, None));
    }

    let (s, prefix) = strip_prefix(s)?;
    let unit = find_unit(s)?;
    if !unit.is_metric() {
        return None;
    }

    Some((unit, Some(prefix)))
}

#[cfg(test)]
mod test {
    use super::get;

    #[test]
    fn test_metric_prefix() {
        let unit = get("kilometer").unwrap();
        assert_eq!(unit.0.name(), "meter");
        assert_eq!(unit.1.unwrap().name, "kilo");

        let unit = get("km").unwrap();
        assert_eq!(unit.0.name(), "meter");
        assert_eq!(unit.1.unwrap().name, "kilo");
    }
}
