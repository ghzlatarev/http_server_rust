// We don't need the mod here 
// because rust will automatically create it
// because the name of the file is "server"

use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use super::thread_pool::ThreadPool;
use super::website_handler::RequestHandler;
use closure::closure;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

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
    pub fn run(self, public_path: String) {
        println!("Listening on {}", self.addr);

        let listener:TcpListener = TcpListener::bind(&self.addr).unwrap();

        let thread_count = 2;
        let pool = ThreadPool::new(thread_count);

        let public_path = Arc::new(public_path);
        // Writing "loop" is the same as "while true"
        loop {
            println!("Looping the loop \n");
            match listener.accept() {
                // We can substitute any and all of the names of variables in the tuple with an underscore in order to ignore them
                Ok((stream, _)) => {
                    let closure = closure!(move mut stream, clone public_path, || {
                        let mut handler = RequestHandler::new(public_path);

                        let mut buffer:[u8; 1024] = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {

                                //println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                
                                let test =  String::from_utf8_lossy(&buffer);
                                if test.contains("sleep") {
                                    // This seems to be causing the errors on response.send() below
                                    thread::sleep(Duration::from_secs(20));
                                    println!("The sleep thread slept for 20 seconds. The other thread should have finished by now! \n");
                                }else{
                                    //thread::sleep(Duration::from_secs(10));
                                    println!("The woke thread didn't sleep and should have finished by now! \n");
                                }

                                let response = match Request::try_from(&buffer[..]) {
                                    // Ok(request) =>  handler.handle_request(&request),
                                    // Err(e) =>  handler.handle_bad_request(&e),
                                    Ok(request) =>  {
                                        println!("The request matched Ok \n");
                                        handler.handle_request(&request)
                                    },
                                    Err(e) => {
                                        println!("The request matched to Err \n");
                                        handler.handle_bad_request(&e)
                                    },
                                };
                            
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response: {}", e);
                                }
                                // Below line does the same
                                //let res: &Result<Request, _> = &buffer[..].try_into();
                            
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    });
                
                    pool.execute(closure);
                    
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
