use afire::Request;

pub mod open_ai;

pub struct Completion {
    pub content_type: String,
    pub body: Vec<u8>,
    pub tokens: u64,
}

pub trait Completer {
    fn complete(&self, req: &Request) -> anyhow::Result<Completion>;
}
