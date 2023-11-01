use lwwsws::{HttpServer, Route};

fn main() {
    let server = HttpServer::new()
        .add_route(Route::new("/".to_owned(), || {
            "This is the index page.".to_owned()
        }))
        .add_route(Route::new("/hello".to_owned(), || {
            "Hello World!".to_owned()
        }));

    server.run(("127.0.0.1", 8080));
}
