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
    #[props(default = false)]
    pub error: bool,
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
        error,
        oninput,
        kind,
    }: TextFieldProps,
) -> Element {
    let oninput = move |evt| oninput.unwrap_or_default().call(evt);
    rsx! {
        input {
            class: "bg-primary-50 border-2 rounded placeholder:text-sm text-sm w-full px-2 py-3",
            class: if error {
                "border-danger-500 text-danger-500 placeholder:text-danger-500"
            } else {
                "border-primary-500 text-primary-500 placeholder:text-primary-500"
            },
            r#type: kind.to_string(),
            name: name,
            value: value,
            placeholder: placeholder,
            disabled: disabled,
            oninput: oninput
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub label: Option<String>,
    #[props(into)]
    pub value: Option<bool>,
    #[props(default = false)]
    pub disabled: bool,
    pub oninput: Option<EventHandler<FormEvent>>,
}

pub fn Checkbox(
    CheckboxProps {
        name,
        label,
        value,
        disabled,
        oninput,
    }: CheckboxProps,
) -> Element {
    let oninput = move |evt| oninput.unwrap_or_default().call(evt);

    rsx! {
        input {
            r#type: "checkbox",
            class: r#"accent-primary-500 rounded border border-primary-500 w-5 h-5"#,
            name: name,
            "aria-label": label,
            value: value,
            disabled: disabled,
            oninput: oninput
        }
    }
}
