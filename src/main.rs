#![allow(dead_code)]

use server::Server;
use std::env;

// This is like copy pasting the contents of the server module into this file
mod server;
mod http;
mod request_handler;
mod thread_pool;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(public_path);

    println!("Shutting down.");
}