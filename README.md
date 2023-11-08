# lwwsws

`lwwsws` is a very basic http webserver based on strings, that only uses the standard rust library. 

### Example
Basic server using async functions.

```rust
use lwwsws::HttpServer;

async fn get_string() -> String {
    "Hello, World!".to_string()
}

#[tokio::main]
async fn main() {
    let handler = |path: String| async move {
        match path.as_str() {
            "/" => Some(get_string().await),
            "/hello" => Some("Wow, this is a new route!".to_string()),
            _ => None,
        }
    };

    let http_server = HttpServer::new(handler);

    http_server.run("127.0.0.1:8080").await;
}
```
