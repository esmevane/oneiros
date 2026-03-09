use askama::Template;
use std::net::SocketAddr;

#[derive(Template)]
#[template(path = "mcp.txt")]
pub struct McpTemplate<'a> {
    pub addr: &'a SocketAddr,
    pub token: &'a str,
}

impl<'a> McpTemplate<'a> {
    pub fn new(addr: &'a SocketAddr, token: &'a str) -> Self {
        Self { addr, token }
    }
}
