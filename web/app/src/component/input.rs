use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldKind {
    Text,
    Password,
}

impl Display for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::Password => "password",
        };
        s.fmt(f)
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TextFieldProps {
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub value: Option<String>,
    #[props(into)]
    pub placeholder: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = FieldKind::Text)]
    pub kind: FieldKind,
    pub oninput: Option<EventHandler<FormEvent>>,
}

pub fn TextField(
    TextFieldProps {
        name,
        value,
        placeholder,
        disabled,
        oninput,
        kind,
    }: TextFieldProps,
) -> Element {
    let oninput = move |evt| oninput.unwrap_or_default().call(evt);
    rsx! {
        input {
            class: "bg-primary-50 border-2 rounded border-primary-500 placeholder:text-primary-500 placeholder:text-sm text-sm text-primary-500 w-full px-2 py-3",
            r#type: kind.to_string(),
            name: name,
            value: value,
            placeholder: placeholder,
            disabled: disabled,
            oninput: oninput
        }
    }
}
