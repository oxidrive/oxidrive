use vite_rs::Embed;

#[derive(Embed)]
#[root = "."]
#[output = "./build"]
#[manifest = "./.svelte-kit/output/server/.vite/manifest.json"]
#[dev_server_port = "5173"]
pub struct Assets;

#[cfg(debug_assertions)]
pub fn start_dev_server() -> Option<vite_rs::ViteProcess> {
    tracing::info!("vite dev server started");
    Assets::start_dev_server()
}
