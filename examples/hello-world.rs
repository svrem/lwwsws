use lwwsws::HttpServer;

async fn handler(path: String) -> Option<String> {
    match path.as_str() {
        "/" => Some("Hello, World!".to_string()),
        "/hello" => Some("Wow, this is a new route!".to_string()),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let http_server = HttpServer::new(handler);

    http_server.run("127.0.0.1:8080").await
}
