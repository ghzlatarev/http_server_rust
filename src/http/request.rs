use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    // super means go 1 level up
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self ) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // The compiler will go ahead and implement TryInto on &[u8] automatically
    // This is basically adding functionality to a library that I didn't write
    // When we call try_from we are going to pass some reference to some memory
    // and what will be returned has the same lifetime as the reference passed as a parameter
    // The compiler now knows that whatever the funciton returns has a relationship with the passed buffer
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        // The ? at the end is like matching but if there's an error we will just exit the function with it.
        let request = str::from_utf8(buf)?;
        
        // GET /search?name=abs&sort=1 HTTP/1.1\r\n...HEADERS...
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        
        let mut query_string = None;
        // The if let feature lets us only match on the variants we care about
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // Loop through all the elements in the iterator
    for (index, character) in request.chars().enumerate() {
        if character == ' '  || character == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }

    None
}

#[derive(Eq, PartialEq)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding, // anything different than utf8
    InvalidProtocol, // for cases with different http version
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl Error for ParseError {

}

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn try_from_valid() {
        let buffer:&[u8] = b"GET /search?name=abs&sort=1 HTTP/1.1\r\n";
        
        match Request::try_from(buffer) {
            Ok(
                Request {
                    path,
                    query_string,
                    method,
                }
            ) => {
                assert_eq!(path, "/search");
                assert_eq!(method, Method::GET);
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn try_from_protocol_error() {
        let buffer:&[u8] = b"GET /search?name=abs&sort=1 HTTASDP/1.1\r\n";
        
        match Request::try_from(buffer) {
            Ok(_) => {
                assert!(false);
            },
            Err(e) => {
                assert_eq!(e, ParseError::InvalidProtocol);
            }
        }
    }

    #[test]
    fn try_from_invalid_method_error() {
        let buffer:&[u8] = b"GE00asT /search?name=abs&sort=1 HTTP/1.1\r\n";
        
        match Request::try_from(buffer) {
            Ok(_) => {
                assert!(false);
            },
            Err(e) => {
                assert_eq!(e, ParseError::InvalidMethod);
            }
        }
    }

    #[test]
    fn try_from_invalid_request_error() {
        let buffer:&[u8] = b"GET HTTP/1.1\r\n";
        
        match Request::try_from(buffer) {
            Ok(_) => {
                assert!(false);
            },
            Err(e) => {
                assert_eq!(e, ParseError::InvalidRequest);
            }
        }
    }
}