use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf_lifetime> {
    path: &'buf_lifetime str,
    query_string: Option<QueryString<'buf_lifetime>>,
    // super means go 1 level up
    method: Method,
}

impl<'buf_lifetime> Request<'buf_lifetime> {
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

impl<'buf_lifetime> TryFrom<&'buf_lifetime [u8]> for Request<'buf_lifetime> {
    type Error = ParseError;

    // The compiler will go ahead and implement TryInto on &[u8] automatically
    // This is basically adding functionality to a library that I didn't write
    // When we call try_from we are going to pass some reference to some memory
    // and what will be returned has the same lifetime as the reference passed as a paramter
    // The compiler now knows that whatever the funciton returns has a relationship with the passed buffer
    fn try_from(buf: &'buf_lifetime [u8]) -> Result<Request<'buf_lifetime>, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }
        
        let request = str::from_utf8(buf)?;
        
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }
        
        // GET /search?name=abs&sort=1 HTTP/1.1\r\n...HEADERS...
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }
        
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
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    // Loop through all the elements in the iterator
    for (index, character) in request.chars().enumerate() {
        if character == ' '  || character == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }

    None
}

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