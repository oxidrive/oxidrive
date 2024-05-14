use dioxus::prelude::*;

use crate::layout::*;
use crate::page::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(AppShell)]
    #[layout(Centered)]
    #[route("/setup")]
    Setup {},
    #[route("/login?:redirect_to")]
    Login { redirect_to: String },
    #[end_layout]
    #[route("/files/:..path")]
    Files { path: Vec<String> },
    #[redirect("/", || Route::Files { path: Vec::new() })]
    #[layout(Centered)]
    #[route("/:..path")]
    NotFound { path: Vec<String> },
}

impl Route {
    pub fn requires_authentication(&self) -> bool {
        !matches!(
            self,
            Self::Setup {} | Self::Login { .. } | Self::NotFound { .. }
        )
    }
}
