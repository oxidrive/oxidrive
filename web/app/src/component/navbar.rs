use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon};

use crate::{
    component::{Logo, LogoColor},
    route::Route,
};

pub fn Navbar() -> Element {
    rsx! {
        nav { class: "flex flex-row items-center justify-between w-full py-2 px-4",
            div { class: "flex flex-row items-center justify-start",
                Link { to: Route::Files { path: Vec::new() }, Logo { height: 50, width: 50, color: LogoColor::White, with_name: true } }
            }
            div { class: "flex flex-row items-center justify-end gap-10",
                Icon { fill: "white", height: 25, width: 25, icon: FaMagnifyingGlass }
                Icon { fill: "white", height: 25, width: 25, icon: FaBell }
                Icon { fill: "white", height: 25, width: 25, icon: FaCircleUser }
            }
        }
    }
}
