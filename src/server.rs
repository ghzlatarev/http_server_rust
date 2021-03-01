// We don't need the mod here 
// because rust will automatically create it
// because the name of the file is "server"

use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        // We can provide default implementation that the implementors are not forced to override.
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server {
            addr: addr
        }
    }

    // The run() function will take ownership of the instance
    // and hence the struct will be deallocated when the function exits.
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener:TcpListener = TcpListener::bind(&self.addr).unwrap();

        // Writing "loop" is the same as "while true"
        loop {
            match listener.accept() {
                // We can substitute any and all of the names of variables in the tuple with an underscore in order to ignore them
                Ok((mut stream, _)) => {
                    let mut buffer:[u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) =>  handler.handle_request(&request),
                                Err(e) =>  handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                            // Below line does the same
                            //let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

            // The below code does the same as the match statement above
            //let res = listener.accept();
            //if res.is_err() {
            //    continue;
            //}
            //let (straem, addr) = res.unwrap();
        }
    }
}  
