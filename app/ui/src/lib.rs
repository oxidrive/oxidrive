#[cfg(feature = "vite-dev-server")]
mod vite {
    use crate::AssetFile;

    #[derive(vite_rs::Embed)]
    #[root = "."]
    #[output = "./build"]
    #[manifest = "./.svelte-kit/output/server/.vite/manifest.json"]
    #[dev_server_port = "5173"]
    pub struct Assets;

    impl AssetFile for vite_rs::ViteFile {
        fn content_type(&self) -> Option<&str> {
            Some(&self.content_type)
        }

        fn last_modified(&self) -> Option<u64> {
            self.last_modified
        }

        fn into_data(self) -> std::borrow::Cow<'static, [u8]> {
            self.bytes.into()
        }
    }
}

#[cfg(not(feature = "vite-dev-server"))]
mod embed {
    use crate::AssetFile;

    #[derive(rust_embed::Embed)]
    #[folder = "./build"]
    pub struct Assets;

    impl AssetFile for rust_embed::EmbeddedFile {
        fn content_type(&self) -> Option<&str> {
            None
        }

        fn last_modified(&self) -> Option<u64> {
            self.metadata.last_modified()
        }

        fn into_data(self) -> std::borrow::Cow<'static, [u8]> {
            self.data
        }
    }
}

#[cfg(feature = "vite-dev-server")]
pub use vite::Assets;

#[cfg(not(feature = "vite-dev-server"))]
pub use embed::Assets;

pub trait AssetFile {
    fn content_type(&self) -> Option<&str>;
    fn last_modified(&self) -> Option<u64>;
    fn into_data(self) -> std::borrow::Cow<'static, [u8]>;
}

#[cfg(feature = "vite-dev-server")]
fn start_dev_server(ctx: app::context::Context) {
    tokio::spawn(async move {
        tracing::info!("vite dev server started");
        let _guard = Assets::start_dev_server();
        ctx.cancelled().await;
        Assets::stop_dev_server();
        tracing::info!("vite dev server stopped");
    });
}

pub struct WebUiModule;

#[app::async_trait]
#[allow(unused_variables)] // ctx is used if vite-dev-server is enabled
impl app::Hooks for WebUiModule {
    async fn after_start(
        &mut self,
        ctx: app::context::Context,
        _c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        #[cfg(feature = "vite-dev-server")]
        start_dev_server(ctx);

        Ok(())
    }
}
