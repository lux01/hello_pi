pub mod http;

use std::fmt;
use std::str;
use std::convert::Into;
use std::str::FromStr;

use url::Url;

macro_rules! api_url {
    ( $x:expr ) => (Url::parse(format!("https://discordapp.com/api{}", $x).as_str()).unwrap());
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    OAuth,
    Bot,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::OAuth => write!(f, "Bearer"),
            TokenType::Bot => write!(f, "Bot"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TokenTypeParseErr;

impl FromStr for TokenType {
    type Err = TokenTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bearer" => Ok(TokenType::OAuth),
            "Bot"    => Ok(TokenType::Bot),
            _        => Err(TokenTypeParseErr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthorizationToken {
    token: String,
    token_type: TokenType,
}

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


#[derive(Debug)]
pub struct Connection {
    auth: AuthorizationToken,
    gateway: Url,
}

impl Connection {
    pub fn from_token<'a, C: http::HttpClient<'a>>(token: String, token_type: TokenType, client: &'a C) -> Connection {
        use discord::http::HttpClient;
        use discord::http::HttpRequest;
        
        let auth = AuthorizationToken {
            token: token,
            token_type: token_type,
        };
        
        let res = client.get(api_url!("/gateway"))
            .header(http::HttpHeader::Authorization(auth.clone()))
            .header(http::HttpHeader::UserAgent(format!("LuxBot (http://github.com/lux01/hello_pi, {})", env!("CARGO_PKG_VERSION")).to_owned()))
            .send()
            .unwrap();
            
        Connection {
            auth: auth,
            gateway: unimplemented!()
        }
    }

    pub fn from_token_and_gateway<T: Into<Url>>(token: String, token_type: TokenType, gateway: T) -> Connection {
        Connection {
            auth: AuthorizationToken {
                token: token,
                token_type: token_type,
            },
            gateway: gateway.into(),
        }
    }
}
