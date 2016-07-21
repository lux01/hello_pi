use super::*;
use super::super::AuthorizationToken;

use std::fmt;

use hyper::header;
use hyper::header::Scheme;
use hyper::client::{Client, RequestBuilder, Response, Body};
use hyper::Error;

impl HttpError for Error { }

impl<'a> HttpClient<'a> for Client {
    type Request = RequestBuilder<'a>;
    type Error = Error;

    fn get(&'a self, url: Url) -> Self::Request {
        self.get(url)
    }
}

impl Scheme for AuthorizationToken {
    fn scheme() -> Option<&'static str> {
        None
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.token)
    }
}


impl<'a> HttpRequest<'a, Error> for RequestBuilder<'a> {
    type Response = Response;

    fn send(self) -> Result<Self::Response, Error> {
        self.send()
    }

    fn header(self, header: HttpHeader) -> Self {
        match header {
            HttpHeader::Authorization(token) => self.header(header::Authorization(token)),
            HttpHeader::UserAgent(ua) => self.header(header::UserAgent(ua)),
        }
    }

    fn body(self, body: &'a str) -> RequestBuilder<'a> {
        self.body(Body::BufBody(body.as_bytes(), body.len()))
    }
}

impl HttpResponse for Response {
    fn status(&self) -> HttpStatus {
        let raw = self.status_raw();
        HttpStatus {
            code: raw.0,
            reason: raw.1.clone().into_owned()
        }
    }
}
