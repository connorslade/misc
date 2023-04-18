use afire::{trace, trace::Level, Content, Method, Response, Server};
use app::App;

mod app;
mod completer;

fn main() {
    trace::set_log_level(Level::Trace);
    let mut server = Server::new("localhost", 8080).state(App::new());

    server.stateful_route(Method::ANY, "**", |app, req| {
        println!("[*] Loading completion for `{}`", req.path);
        let completion = app.completer.complete(&req).unwrap();
        Response::new()
            .content(Content::Custom(&completion.content_type))
            .bytes(&completion.body)
    });

    server.start().unwrap();
}
