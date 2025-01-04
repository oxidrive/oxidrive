use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use serde::Deserialize;
use state::AppState;
use tokio::net::TcpListener;

mod routes;
mod session;
mod state;

mod api;
mod files;
mod ui;

#[derive(Clone)]
pub struct Server {
    addr: SocketAddr,
    state: AppState,
}

impl Server {
    fn new(cfg: Config, state: AppState) -> Self {
        Self {
            addr: SocketAddr::new(cfg.host, cfg.port),
            state,
        }
    }

    pub fn local_address(&self) -> SocketAddr {
        self.addr
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        axum::serve(listener, routes::routes().with_state(self.state.clone())).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    host: IpAddr,
    #[serde(default = "default_port")]
    port: u16,

    secret_key: String,
}

fn default_host() -> IpAddr {
    Ipv6Addr::LOCALHOST.into()
}

fn default_port() -> u16 {
    4000
}

pub struct WebModule;

impl app::Module for WebModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(AppState::new);
        c.bind(Server::new);
    }
}
