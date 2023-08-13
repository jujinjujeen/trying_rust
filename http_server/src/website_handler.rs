use super::http::{Request, Response, StatusCode};
use super::server::Handler;
use crate::http::Method;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traverse attack attempted at: {}", path.display());
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => {
                    Response::new(StatusCode::Ok, Some("<h1>somebody hello</h1>".to_string()))
                }
                path => match self.read_file(path) {
                    Some(file) => Response::new(StatusCode::Ok, Some(file)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
