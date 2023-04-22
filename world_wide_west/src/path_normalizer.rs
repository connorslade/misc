use afire::{prelude::MiddleResult, Middleware, Request};

pub struct PathNormalizer;

impl Middleware for PathNormalizer {
    fn pre(&self, req: &mut Request) -> MiddleResult {
        req.path = req.path.trim_end_matches('/').to_owned();
        MiddleResult::Continue
    }
}
