use super::Size;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Heading {
    H1,
    H2,
    H3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TitleColor {
    Primary,
    PrimaryDark,
    Black,
}

impl TitleColor {
    pub fn class(&self) -> &'static str {
        match self {
            TitleColor::Primary => "text-primary-500",
            TitleColor::PrimaryDark => "text-primary-700",
            TitleColor::Black => "text-black",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FontWeight {
    Regular,
    Bold,
}

impl FontWeight {
    pub fn class(&self) -> &'static str {
        match self {
            FontWeight::Regular => "font-regular",
            FontWeight::Bold => "font-bold",
        }
    }
}

#[component]
pub fn Title(
    #[props(default = Heading::H1)] h: Heading,
    #[props(default = TitleColor::Black)] color: TitleColor,
    #[props(default = Size::Large)] size: Size,
    #[props(default = FontWeight::Regular)] weight: FontWeight,
    #[props(into)] class: Option<String>,
    children: Element,
) -> Element {
    let class = format!(
        "text-{} {} {} {}",
        size.class_suffix(),
        color.class(),
        weight.class(),
        class.unwrap_or_default(),
    );
    match h {
        Heading::H1 => {
            rsx! {
                h1 { class: class, { children } }
            }
        }
        Heading::H2 => {
            rsx! {
                h2 { class: class, { children } }
            }
        }
        Heading::H3 => {
            rsx! {
                h3 { class: class, { children } }
            }
        }
    }
}
