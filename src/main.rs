#![allow(dead_code)]

use server::Server;
use std::env;

// This is like copy pasting the contents of the server module into this file
mod server;
mod http;
mod website_handler;
mod thread_pool;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(public_path);
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
// 1. Extend the request and response. Right now it completely ignores the headers                          
// 2. Optimize the server to utilize mutliple threads                                                       -> Done
// 3. Add more documentation to ThreadPool and its public methods.                                          
// 4. Add tests of the library’s functionality.                                                             
// 5. Change calls to unwrap to more robust error handling.                                                 
// 6. Use ThreadPool to perform some task other than serving web requests.                                  
// 7. Find a thread pool crate on crates.io and implement a similar web server using the                    
//    crate instead. Then compare its API and robustness to the thread pool we implemented.            