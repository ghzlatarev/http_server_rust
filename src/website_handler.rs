use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::fs;
use std::path::Path;
use std::sync::Arc;

// Windows specific
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}

pub struct RequestHandler {
    public_path: Arc<String>
}

impl RequestHandler {
    pub fn new(public_path: Arc<String>) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                let path = adjust_canonicalization(path);
                if path.starts_with(self.public_path.as_ref()) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }

            Err(_) => None,
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }

            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}