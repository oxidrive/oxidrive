#[cfg(not(feature = "skip-assets-build"))]
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

#[cfg(feature = "skip-assets-build")]
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

#[cfg(feature = "skip-assets-build")]
pub use embed::Assets;
#[cfg(not(feature = "skip-assets-build"))]
pub use vite::Assets;

pub trait AssetFile {
    fn content_type(&self) -> Option<&str>;
    fn last_modified(&self) -> Option<u64>;
    fn into_data(self) -> std::borrow::Cow<'static, [u8]>;
}

#[cfg(all(not(feature = "skip-assets-build"), debug_assertions))]
pub fn start_dev_server(enable: bool) -> Option<vite_rs::ViteProcess> {
    if !enable {
        tracing::warn!("vite dev server disabled");
        return None;
    }

    tracing::info!("vite dev server started");
    Assets::start_dev_server()
}

#[cfg(all(feature = "skip-assets-build", debug_assertions))]
pub fn start_dev_server(_enable: bool) -> Option<vite_rs::ViteProcess> {
    None
}
