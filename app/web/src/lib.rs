use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use bytesize::ByteSize;
use cors::CorsConfig;
use serde::Deserialize;
use state::AppState;
use tokio::net::TcpListener;
use tower::{
    layer::util::Identity,
    util::{option_layer, Either},
};
use tower_http::cors::CorsLayer;
use tower_surf::Surf;
use utoipa::openapi::OpenApi;

#[cfg(debug_assertions)]
pub use oxidrive_ui::start_dev_server;

mod cors;
mod paginate;
mod routes;
mod session;
mod state;

mod api;
mod auth;
mod files;
mod ui;

#[derive(Clone)]
pub struct Server {
    addr: SocketAddr,
    state: AppState,
    cfg: Config,
}

impl Server {
    fn new(cfg: Config, state: AppState) -> Self {
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
        axum::serve(
            listener,
            routes::routes(&self.cfg).with_state(self.state.clone()),
        )
        .await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    host: IpAddr,
    #[serde(default = "default_port")]
    port: u16,

    secret_key: String,

    #[serde(default)]
    disable_csrf: bool,

    #[serde(default = "default_upload_body_limit")]
    upload_body_limit: ByteSize,

    #[serde(default)]
    cors: Option<CorsConfig>,
}

impl Config {
    pub(crate) fn csrf(&self) -> Either<Surf, Identity> {
        let surf = Surf::new(&self.secret_key)
            .cookie_name("oxidrive_csrf_token")
            .prefix(false);

        let layer = match self.disable_csrf {
            true => None,
            false => Some(surf),
        };

        option_layer(layer)
    }

    pub(crate) fn cors(&self) -> CorsLayer {
        self.cors.as_ref().map(CorsLayer::from).unwrap_or_default()
    }

    pub fn empty() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            secret_key: Default::default(),
            disable_csrf: Default::default(),
            upload_body_limit: default_upload_body_limit(),
            cors: Default::default(),
        }
    }
}

fn default_host() -> IpAddr {
    Ipv6Addr::LOCALHOST.into()
}

fn default_port() -> u16 {
    4000
}

fn default_upload_body_limit() -> ByteSize {
    ByteSize::gb(10)
}

pub struct WebModule;

impl app::Module for WebModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(AppState::new);
        c.bind(Server::new);
    }
}

pub fn openapi_schema() -> OpenApi {
    let (_, api) = routes::openapi_router(&Config::empty()).split_for_parts();
    api
}
