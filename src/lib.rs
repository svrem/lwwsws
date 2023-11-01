use std::{
    future::Future,
    io::{Read, Write},
    net::{TcpListener, ToSocketAddrs},
    pin::Pin,
};

// type RouteFunc = Fn() -> Box<dyn Future<Output = String>>;
pub trait RouteFunc: Fn() -> Pin<Box<dyn Future<Output = String>>> {}

pub struct Route {
    path: String,
    func: Box<dyn RouteFunc>,
}

impl Route {
    pub fn new(path: String, func: impl RouteFunc + 'static) -> Self
// where
        // F: RouteFunc,
    {
        Self {
            func: Box::from(func),
            path,
        }
    }
}

pub struct HttpServer {
    routes: Vec<Route>,
}

impl HttpServer {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(mut self, route: Route) -> Self {
        self.routes.push(route);

        self
    }

    pub async fn run<Addr>(&self, addr: Addr)
    where
        Addr: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr).unwrap();

        'incoming: for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let req: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
            let (_, path) = match get_method_and_path_from_request(req.to_string()) {
                Some((method, path)) => (method, path),
                None => {
                    continue 'incoming;
                }
            };

            for route in &self.routes {
                if path == route.path {
                    // run the route's function
                    let response = (route.func)().await;

                    let response_string = format!("HTTP/2 200\r\n\r\n{}", response);
                    stream.write_all(response_string.as_bytes()).unwrap();
                    continue 'incoming;
                }
            }

            stream
                .write_all("HTTP/2 404\r\n\r\nNot Found".as_bytes())
                .unwrap();
        }
    }
}

fn get_method_and_path_from_request(req: String) -> Option<(String, String)> {
    let req_lines: Vec<&str> = req.split("\r\n").collect();

    let splitted_method_path_http = req_lines[0].split(" ").collect::<Vec<&str>>();

    if splitted_method_path_http.len() != 3 {
        return None;
    }

    Some((
        splitted_method_path_http[0].to_string(),
        splitted_method_path_http[1].to_string(),
    ))
}
