use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::HTTP => write!(f, "http"),
            Protocol::HTTPS => write!(f, "https"),
            Protocol::SOCKS4 => write!(f, "socks4"),
            Protocol::SOCKS5 => write!(f, "socks5"),
        }
    }
}

impl Protocol {
    pub fn from_str(s: &str) -> Self {
        match s {
            "http" => Protocol::HTTP,
            "https" => Protocol::HTTPS,
            "socks4" => Protocol::SOCKS4,
            "socks5" => Protocol::SOCKS5,
            _ => Protocol::HTTP,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Proxy {
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.username.clone(), self.password.clone()) {
            (Some(username), Some(password)) => {
                write!(
                    f,
                    "{}:{}:{}:{}:{}",
                    self.protocol, self.host, self.port, username, password,
                )
            }
            _ => write!(f, "{}:{}:{}", self.protocol, self.host, self.port),
        }
    }
}
