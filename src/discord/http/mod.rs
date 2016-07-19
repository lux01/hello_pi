mod hyper;

pub use url::Url;
use std::error::Error as StdError;
use std::fmt;
use std::io::Read;

use super::AuthorizationToken;

pub trait HttpClient<'a> {
    type Request: HttpRequest<'a>;

    fn get(&'a self, url: Url) -> Self::Request;
}

pub enum HttpHeader<'a> {
    Authorization(&'a AuthorizationToken),
    UserAgent(String),
}

pub trait HttpRequest<'a> {
    type Response: HttpResponse;
    type Error: StdError + fmt::Debug;

    fn header(self, HttpHeader) -> Self;
    fn body(self, header: &'a str) -> Self;
    fn send(self) -> Result<Self::Response, Self::Error>;
}

pub struct HttpStatus {
    pub code: u16,
    pub reason: String,
}

pub trait HttpResponse: Read {
    type Error: StdError + fmt::Debug;

    fn status(&self) -> HttpStatus;
}
