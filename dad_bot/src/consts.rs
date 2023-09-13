use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Use \bi('?| ?a)m\b to require a word boundary on both sides.
    pub static ref IM_REGEX: Regex = Regex::new(r"i'?m").unwrap();
    pub static ref DAD_REGEX: Regex = Regex::new(r"\bi('?m| *am) *dad\b").unwrap();
    pub static ref SHUT_REGEX: Regex = Regex::new(r"shut").unwrap();
}

/// The amount of time in seconds that a message is considered "dadable" for.
/// Set to 0 to disable.
pub const DAD_TIMEOUT: u64 = 10;

/// Automatically say "(shut)" to dadable messages.
pub const AUTO_SHUT: bool = true;
