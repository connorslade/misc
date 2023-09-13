use anyhow::Result;

mod content_at_scale;
mod writer;
mod zerogpt;

pub trait Checker {
    /// Service name
    fn name(&self) -> &'static str;
    /// Returns the probability that the text is fake
    fn check(&self, text: &str) -> Result<f32>;

    /// Minimum number of characters
    fn min(&self) -> u32;
    /// Maximum number of characters
    fn max(&self) -> u32;
}

pub const CHECKERS: &[&(dyn Checker + Send + Sync)] = &[
    &zerogpt::ZeroGPT,
    &content_at_scale::ContentAtScale,
    &writer::Writer,
];
