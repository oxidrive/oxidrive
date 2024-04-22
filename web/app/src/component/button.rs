use dioxus::prelude::*;

const CLASS_PRIMARY: &str =
    "bg-primary-500 text-primary-50 py-2 px-6 flex flex-row content-stretch items-center justify-center rounded";

const CLASS_GHOST: &str =
    "text-primary-500 underline p-1 flex flex-row content-stretch items-center justify-center rounded";

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
        button { class: class.into(), onclick: onclick, {children} }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ButtonLinkProps {
    #[props(default = ButtonVariant::Filled)]
    pub variant: ButtonVariant,
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
        variant: _,
        children,
        new_tab,
        onclick,
        onclick_only,
        rel,
        to,
    }: ButtonLinkProps,
) -> Element {
    rsx! {
        Link {
            class: class.into(),
            new_tab,
            onclick,
            onclick_only,
            rel,
            to,
            {children}
        }
    }
}
