use dioxus::prelude::*;
use oxidrive_api::Oxidrive;

pub fn use_oxidrive_api() -> Signal<Oxidrive> {
    use_context()
}

pub async fn init() {
    let origin = web_sys::window()
        .expect("window not found")
        .location()
        .origin()
        .unwrap();
    let api = Oxidrive::new(origin);
    use_context_provider(|| Signal::new(api));
}
