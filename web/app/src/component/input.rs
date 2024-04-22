use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TextFieldProps {
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub value: String,
    #[props(into)]
    pub placeholder: String,
    #[props(default = false)]
    pub disabled: bool,
    pub oninput: EventHandler<FormEvent>,
}

pub fn TextField(
    TextFieldProps {
        name,
        value,
        placeholder,
        disabled,
        oninput,
    }: TextFieldProps,
) -> Element {
    let oninput = move |evt| oninput.call(evt);

    rsx! {
        input {
            class: "bg-primary-50 border-2 rounded border-primary-500 placeholder:text-primary-500 placeholder:text-sm text-sm text-primary-500 w-full px-2 py-3",
            name: name,
            value: value,
            placeholder: placeholder,
            disabled: disabled,
            oninput: oninput
        }
    }
}
