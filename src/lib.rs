use std::{
    future::Future,
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct HttpServer<F, Fut>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Option<String>>,
{
    handler: F,
}

impl<F, Fut> HttpServer<F, Fut>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Option<String>>,
{
    pub fn new(handler: F) -> Self
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Option<String>>,
    {
        Self { handler }
    }

    pub async fn run<Addr>(&self, addr: Addr) -> Result<(), std::io::Error>
    where
        Addr: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr)?;

        'incoming: for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue 'incoming,
            };

            let mut buffer = [0; 1024];
            if stream.read(&mut buffer).is_err() {
                write_500_error(&mut stream).ok();
                continue 'incoming;
            }

            let req: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
            let (_, path) = match get_method_and_path_from_request(req.to_string()) {
                Some((method, path)) => (method, path),
                None => {
                    continue 'incoming;
                }
            };

            let res = (self.handler)(path).await;

            if let Some(http_response) = res {
                let response_string = format!("HTTP/2 200\r\n\r\n{}", http_response);

                if stream.write_all(response_string.as_bytes()).is_err() {
                    write_500_error(&mut stream).ok();
                }

                continue 'incoming;
            }

            write_404_error(&mut stream).ok();
        }

        Ok(())
    }
}

fn write_404_error(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    stream.write_all("HTTP/2 404\r\n\r\nNot Found".as_bytes())?;

    Ok(())
}

fn write_500_error(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    stream.write_all("HTTP/2 500\r\n\r\nInternal Server Error".as_bytes())?;

    Ok(())
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
