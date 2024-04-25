use dioxus::prelude::*;
use log::LevelFilter;
use oxidrive::App;
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}
