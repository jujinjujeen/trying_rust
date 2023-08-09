use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
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
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer[..size]));
                            match Request::try_from(&buffer[..size]) {
                                Ok(request) => {
                                    println!("{}", request.path);
                                },
                                Err(err) => println!("Error parsing request: {}", err),
                            }
                        },
                        Err(err) => println!("Reading request failed: {}", err),
                    }
                },
                Err(err) => {
                    println!("Failed to establish a connection: {}", err);
                }
            }
        }
    }
}
