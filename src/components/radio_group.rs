use dioxus::prelude::*;
use lucide_rust::dioxus::circle_icon::Circle;

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            role: "radiogroup",
            class: "grid gap-3 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupItemProps {
    class: Option<String>,
    checked: bool,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let base = "border-input text-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 aspect-square size-4 shrink-0 rounded-full border shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50";

    rsx! {
        label {
            class: "inline-flex items-center gap-2",
            input {
                r#type: "radio",
                class: "{base} {class}",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                }
            }
            if props.checked {
                span {
                    class: "relative flex items-center justify-center",
                    Circle {
                        class: "fill-primary absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2"
                    }
                }
            }
        }
    }
}
