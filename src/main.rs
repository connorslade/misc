use afire::{trace, trace::Level, Content, Method, Response, Server};
use app::App;
use completer::Completion;

use crate::database::Database;

mod app;
mod completer;
mod database;

fn main() {
    trace::set_log_level(Level::Trace);
    let mut server = Server::new("localhost", 8080).state(App::new());

    server.stateful_route(Method::ANY, "**", |app, req| {
        let db = app.db();
        let completion = match db.get_completion(&req.path) {
            Some(i) => i,
            None => {
                println!("[*] Loading completion for `{}`", req.path);
                let out = app.completer.complete(&req).unwrap();
                db.set_completion(&req.path, &out);
                out
            }
        };

        res(&completion)
    });

    server.start().unwrap();
}

fn res(completion: &Completion) -> Response {
    Response::new()
        .content(Content::Custom(&completion.content_type))
        .bytes(&completion.body)
}
