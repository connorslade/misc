use anyhow::Result;

mod zerogpt;

pub trait Checker {
    fn name(&self) -> &'static str;
    fn check(&self, text: &str) -> Result<f32>;

    fn min(&self) -> u32;
    fn max(&self) -> u32;
}

pub const CHECKERS: &[&dyn Checker] = &[&zerogpt::ZeroGPT];
