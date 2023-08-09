use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    pub path: String,
    query: Option<String>,
    method: Method,
}

impl Request {
    
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}