use std::io::{Write, Result as IoResult};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

    // impl Write implies Static Dispatch, meaning the compiler will check all instances of calls to this function,
    // infer a concrete type for each one and write out a concrete implementation for that type.
    // This way we don't need to use a run-time vtable to lookup which function address we need. 
    pub fn send (&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}