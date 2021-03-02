#![allow(dead_code)]

use server::Server;
use std::env;
use thread_pool::ThreadPool;
use website_handler::WebsiteHandler;

// This is like copy pasting the contents of the server module into this file
mod server;
mod http;
mod website_handler;
mod thread_pool;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

/*
GET /user?id=10 HTTP/1.1 \r\n
HEADERS \r\n
BODY
*/

// I learned about:
// 1. structs
// 2. enums
// 3. traits
// 4. networking
// 5. data structures
// 6. lifetimes

// TODO:: 
// 1. Extend the request and response. Right now it completely ignores the headrs
// 2. Optimize the server to utilize mutliple threds
//    std::sync and std::thread 