use afire::{trace, trace::Level, Content, Method, Response, Server};
use app::App;

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
                let out = app.completer.complete(req).unwrap();
                db.set_completion(&req.path, &out);
                out
            }
        };

        Response::new()
            .content(Content::Custom(&completion.content_type))
            .header("X-Tokens-Used", completion.tokens.to_string())
            .bytes(&completion.body)
    });

    server.start_threaded(4).unwrap();
}
