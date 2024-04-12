use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LogoProps {
    #[props(default = 100)]
    height: i64,
    #[props(default = 100)]
    width: i64,
    #[props(default = LogoColor::Primary)]
    color: LogoColor,
    #[props(default = false)]
    with_name: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LogoColor {
    Primary,
    White,
}

pub fn Logo(
    LogoProps {
        height,
        width,
        color,
        with_name,
    }: LogoProps,
) -> Element {
    let src = match (color, with_name) {
        (LogoColor::Primary, true) => "/logo-with-name.svg",
        (LogoColor::Primary, false) => "/logo.svg",
        (LogoColor::White, true) => "/logo-white-with-name.svg",
        (LogoColor::White, false) => "/logo-white.svg",
    };

    rsx! { img { src: src, height: height, width: width } }
}
