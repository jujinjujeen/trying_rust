use super::status_code::StatusCode;
use std::io::{Write, Result as IoResult};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(val) => val,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body.len(),
            body,
        )
    }
}
