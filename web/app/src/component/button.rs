use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

const CLASS_PRIMARY: &str = "bg-primary-500 text-primary-50 py-2 px-6 flex flex-row content-stretch items-center justify-center rounded";
const CLASS_GHOST: &str = "text-primary-500 underline p-1 flex flex-row content-stretch items-center justify-center rounded";

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Filled)]
    pub variant: ButtonVariant,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Filled,
    Ghost,
}

pub fn Button(props: ButtonProps) -> Element {
    match props.variant {
        ButtonVariant::Filled => button(CLASS_PRIMARY, props),
        ButtonVariant::Ghost => button(CLASS_GHOST, props),
    }
}

fn button(
    class: impl Into<String>,
    ButtonProps {
        variant: _,
        onclick,
        children,
    }: ButtonProps,
) -> Element {
    let onclick = move |evt| onclick.unwrap_or_default().call(evt);
    rsx! {
        button { class: class.into(), onclick: onclick, { children } }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonColor {
    Primary,
    PrimaryDark,
    White,
}

impl ButtonColor {
    fn class(&self) -> &'static str {
        match self {
            Self::Primary => "text-primary-500",
            Self::PrimaryDark => "text-primary-600",
            Self::White => "text-primary-50",
        }
    }

    fn default_for(variant: ButtonVariant) -> Self {
        match variant {
            ButtonVariant::Filled => Self::White,
            ButtonVariant::Ghost => Self::Primary,
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ButtonLinkProps {
    #[props(default = ButtonVariant::Filled)]
    pub variant: ButtonVariant,
    pub color: Option<ButtonColor>,
    pub children: Element,
    #[props(default)]
    pub new_tab: bool,
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    /// Whether the default behavior should be executed if an `onclick` handler is provided.
    ///
    /// 1. When `onclick` is [`None`] (default if not specified), `onclick_only` has no effect.
    /// 2. If `onclick_only` is [`false`] (default if not specified), the provided `onclick` handler
    ///    will be executed after the links regular functionality.
    /// 3. If `onclick_only` is [`true`], only the provided `onclick` handler will be executed.
    pub onclick_only: bool,
    /// The rel attribute for the generated HTML anchor tag.
    ///
    /// For external `a`s, this defaults to `noopener noreferrer`.
    pub rel: Option<String>,
    /// The navigation target. Roughly equivalent to the href attribute of an HTML anchor tag.
    #[props(into)]
    pub to: IntoRoutable,
}

pub fn ButtonLink(props: ButtonLinkProps) -> Element {
    let button_variant = props.variant;
    match button_variant {
        ButtonVariant::Filled => button_link(CLASS_PRIMARY, props),
        ButtonVariant::Ghost => button_link(CLASS_GHOST, props),
    }
}

fn button_link(
    class: impl Into<String>,
    ButtonLinkProps {
        variant,
        color,
        children,
        new_tab,
        onclick,
        onclick_only,
        rel,
        to,
    }: ButtonLinkProps,
) -> Element {
    let color = color.unwrap_or_else(|| ButtonColor::default_for(variant));
    let class = format!("{} {}", class.into(), color.class());

    rsx! {
        Link {
            class: class,
            new_tab,
            onclick,
            onclick_only,
            rel,
            to,
            { children }
        }
    }
}

#[component]
pub fn Fab<I: IconShape + Clone + PartialEq + 'static>(
    label: String,
    icon: I,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            "aria-label": label,
            class: "rounded-full bg-primary-500 p-2 fixed z-50 bottom-0 right-0 m-8",
            onclick: move |evt| onclick.call(evt),
            Icon { fill: "white", height: 30, width: 30, icon: icon }
            {children}
        }
    }
}
