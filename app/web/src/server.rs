use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::{routes, state::AppState, Config};

#[derive(Clone)]
pub struct Server {
    addr: SocketAddr,
    state: AppState,
    cfg: Config,
}

impl Server {
    pub(crate) fn new(cfg: Config, state: AppState) -> Self {
        Self {
            addr: SocketAddr::new(cfg.host, cfg.port),
            state,
            cfg,
        }
    }

    pub fn local_address(&self) -> SocketAddr {
        self.addr
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        axum::serve(listener, routes::routes(&self.cfg, self.state.clone())).await
    }
}
