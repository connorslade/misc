use std::borrow::Cow;

use crate::Section;

pub trait Splitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str>;
    fn should_split(&self, section: &Section) -> bool;
}
