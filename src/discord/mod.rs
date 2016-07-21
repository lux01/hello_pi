//! Discord
//! =======
//!
//! This module provides a Rust wrapper around a subset
//! of the Discord chat service API. Authorization tokens
//! must be generated externally, either by OAuth2 calls,
//! or by using a pregenerated Bot token.

#![warn(missing_docs)]
pub mod http;

use std::fmt;
use std::convert::Into;
use std::str::FromStr;
use std::io::Error as IoError;

use url::Url;
use url::ParseError as UrlParseError;

pub use serde_json as json;
pub use self::json::error::Error as JsonError;

/// The error wrapper type for the Discord library. The
/// error type for the `http` library is left generic.
#[derive(Debug)]
pub enum DiscordErr<HttpErr: http::HttpError> {
    /// An error occured with the HTTP client
    Http(HttpErr),

    /// A URL failed to parse
    UrlParse(UrlParseError),

    /// An IO error
    Io(IoError),

    /// A JSON error occured
    Json(JsonError),

    /// A JSON value did not have the expected type
    JsonWrongType(&'static str),

    /// A JSON object did not have the expected key
    JsonMissingKey(&'static str),
}

impl<E: http::HttpError> From<E> for DiscordErr<E> {
    fn from(e: E) -> Self {
        DiscordErr::Http(e)
    }
}

impl<E: http::HttpError> From<UrlParseError> for DiscordErr<E> {
    fn from(e: UrlParseError) -> Self {
        DiscordErr::UrlParse(e)
    }
}

impl<E: http::HttpError> From<IoError> for DiscordErr<E> {
    fn from(e: IoError) -> Self {
        DiscordErr::Io(e)
    }
}

impl<E: http::HttpError> From<JsonError> for DiscordErr<E> {
    fn from(e: JsonError) -> Self {
        DiscordErr::Json(e)
    }
}
fn api_url<E: http::HttpError>(x: &str) -> Result<Url, DiscordErr<E>> {
    let url_str = format!("https://discordapp.com/api{}", x);
    Url::parse(&url_str).map_err(From::from)
}


/// Represents whether the associated `AuthorizationToken`
/// is for a user or a bot.
#[derive(Debug, Clone, Copy)]
enum TokenType {
    OAuth,
    Bot,
}

// We need to implement both `fmt::Display` and `FromStr`
// for TokenType so that we can use it as a Scheme in a
// `hyper::headers::Authorization` struct.
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::OAuth => write!(f, "Bearer"),
            TokenType::Bot => write!(f, "Bot"),
        }
    }
}

/// An empty error type to satisfy the `FromStr` trait.
#[derive(Debug, Clone, Copy)]
pub struct TokenTypeParseErr;

impl FromStr for TokenType {
    type Err = TokenTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bearer" => Ok(TokenType::OAuth),
            "Bot" => Ok(TokenType::Bot),
            _ => Err(TokenTypeParseErr),
        }
    }
}

/// An authorization token for a connection to the discord API.
/// This token is not validated client side for validity.
#[derive(Debug, Clone)]
pub struct AuthorizationToken {
    token: String,
    token_type: TokenType,
}

impl AuthorizationToken {
    /// Constructs a new `AuthorizationToken` representing a bot
    /// given the token string.
    pub fn new_bot(token: String) -> AuthorizationToken {
        AuthorizationToken::new(token, TokenType::Bot)
    }

    /// Constructs a new `AuthorizationToken` representing a user
    /// given the token string.
    pub fn new_user(token: String) -> AuthorizationToken {
        AuthorizationToken::new(token, TokenType::OAuth)
    }

    fn new(token: String, token_type: TokenType) -> AuthorizationToken {
        AuthorizationToken {
            token: token,
            token_type: token_type,
        }
    }
}

// Again we need to implement FromStr for AuthorizationToken
// to satisfy a trait bound in the `hyper` library.
impl FromStr for AuthorizationToken {
    type Err = TokenTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(TokenTypeParseErr)
        } else {
            let token_type = try!(parts[0].parse());
            let token = parts[1].to_owned();

            Ok(AuthorizationToken {
                token: token,
                token_type: token_type,
            })
        }

    }
}


/// A set of connection parameters for the Discord HTTP and WebSocket APIs.
/// The connections themselves are not maintained by the interface.
#[derive(Debug)]
pub struct Connection {
    auth: AuthorizationToken,
    gateway: Url,
}

impl Connection {
    /// Creates a new `Connection` from an `AuthorizationToken` by querying
    /// the API for a new gateway using the `HttpClient`.
    pub fn from_token<'a, C: http::HttpClient<'a>>(auth: AuthorizationToken,
                                                   client: &'a C)
                                                   -> Result<Connection, DiscordErr<C::Error>> {
        use discord::http::HttpRequest;
        use std::io::Read;

        // Request a gateway URL from the server
        let mut gateway_json_str = String::new();
        let mut response = try!(client.get(try!(api_url("/gateway")))
            .header(http::HttpHeader::Authorization(auth.clone()))
            .header(http::HttpHeader::UserAgent(format!("LuxBot (http://github.\
                                                         com/lux01/hello_pi, {})",
                                                        env!("CARGO_PKG_VERSION"))
                .to_owned()))
            .send());

        try!(response.read_to_string(&mut gateway_json_str));

        // Parse the resulting JSON
        let data: json::Value = try!(json::from_str(&gateway_json_str));
        let obj = try!(data.as_object().ok_or(DiscordErr::JsonWrongType("Object")));
        let url_str = try!(try!(obj.get("url")
                .ok_or(DiscordErr::JsonMissingKey("url")))
            .as_string()
            .ok_or(DiscordErr::JsonWrongType("String")));

        Ok(Connection {
            auth: auth,
            gateway: try!(Url::parse(url_str)),
        })
    }

    /// Creates a new `Connection` from an `AuthorizationToken` and a cached gateway `Url`.
    pub fn from_token_and_gateway<T: Into<Url>>(auth: AuthorizationToken,
                                                gateway: T)
                                                -> Connection {
        Connection {
            auth: auth,
            gateway: gateway.into(),
        }
    }
}
