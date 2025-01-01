use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use serde::Deserialize;
use tokio::net::TcpListener;

mod routes;

#[derive(Clone)]
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    fn new(cfg: Config) -> Self {
        Self {
            addr: SocketAddr::new(cfg.host, cfg.port),
        }
    }

    pub fn local_address(&self) -> SocketAddr {
        self.addr
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        axum::serve(listener, routes::routes()).await
    }
}

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    host: IpAddr,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_host() -> IpAddr {
    Ipv6Addr::LOCALHOST.into()
}

fn default_port() -> u16 {
    4000
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

pub struct WebModule;

impl app::Module for WebModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(Server::new);
    }
}
