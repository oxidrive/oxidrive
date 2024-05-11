use dioxus::prelude::*;
use oxidrive_api::instance::Status;

pub fn use_instance_status() -> Signal<Status> {
    use_context()
}

pub fn init() {
    use_context_provider(|| Signal::new(Status::default()));
}
