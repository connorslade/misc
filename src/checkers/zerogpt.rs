use super::Checker;

pub struct ZeroGPT;

impl Checker for ZeroGPT {
    fn name(&self) -> &'static str {
        "ZeroGPT"
    }

    fn check(&self, text: &str) -> anyhow::Result<f32> {
        todo!()
    }

    fn min(&self) -> u32 {
        0
    }

    fn max(&self) -> u32 {
        15_000
    }
}
