use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Error parsing request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // arbitrary number of bytes chosen for buffer
                    // it'll certainly hold most of the test requests
                    // but the production app needs more robust solution
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            println!(
                                "Received a request: {}",
                                String::from_utf8_lossy(&buffer[..size])
                            );
                            let response = match Request::try_from(&buffer[..size]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(err) => {
                                    handler.handle_bad_request(&err)
                                }
                            };
                            if let Err(err) = response.send(&mut stream) {
                                println!("Error sending request: {}", err);
                            }
                        }
                        Err(err) => println!("Reading request failed: {}", err),
                    }
                }
                Err(err) => {
                    println!("Failed to establish a connection: {}", err);
                }
            }
        }
    }
}
