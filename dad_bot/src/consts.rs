use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref IM_REGEX: Regex = Regex::new(r"\bi'?a?m\b").unwrap();
    pub static ref DAD_REGEX: Regex = Regex::new(r"\bi('?m| *am) *dad\b").unwrap();
}

/// The amount of time in seconds that a message is considered "dadable" for.
pub const DAD_TIMEOUT: u64 = 10;
