use dioxus::prelude::*;

use super::Size;

#[derive(Clone, Copy, PartialEq)]
pub enum Heading {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TitleColor {
    Primary,
    White,
    Black,
}

impl TitleColor {
    pub fn class(&self) -> &'static str {
        match self {
            TitleColor::Primary => "text-primary-500",
            TitleColor::White => "text-white",
            TitleColor::Black => "text-black",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FontWeight {
    Light,
    Regular,
    Bold,
}

impl FontWeight {
    pub fn class(&self) -> &'static str {
        match self {
            FontWeight::Light => "font-light",
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
        class.unwrap_or_default()
    );

    match h {
        Heading::H1 => rsx! {
            h1 { class: class, {children} }
        },
        Heading::H2 => rsx! {
            h2 { class: class, {children} }
        },
        Heading::H3 => rsx! {
            h3 { class: class, {children} }
        },
        Heading::H4 => rsx! {
            h4 { class: class, {children} }
        },
        Heading::H5 => rsx! {
            h5 { class: class, {children} }
        },
        Heading::H6 => rsx! {
            h6 { class: class, {children} }
        },
    }
}
