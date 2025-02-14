use std::net::{IpAddr, Ipv6Addr};

use bytesize::ByteSize;
use cors::CorsConfig;
use oxidrive_ui::WebUiModule;
use serde::Deserialize;
use state::AppState;
use tower::{
    layer::util::Identity,
    util::{option_layer, Either},
};
use tower_http::cors::CorsLayer;
use tower_surf::Surf;
use utoipa::openapi::OpenApi;

pub use server::Server;

mod cors;
mod headers;
mod paginate;
mod routes;
mod server;
mod session;
mod state;

mod api;
mod auth;
mod files;
mod ui;

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

pub fn openapi_schema() -> OpenApi {
    let (_, api) = routes::openapi_router(&Config::empty()).split_for_parts();
    api
}

#[derive(Clone)]
pub struct WebModule;

impl app::Module for WebModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(AppState::new);
        c.bind(Server::new);
    }
}

#[app::async_trait]
impl app::Hooks for WebModule {
    async fn after_start(
        &mut self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        WebUiModule.after_start(ctx, c).await
    }
}
