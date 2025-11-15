use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    class: Option<String>,
    #[props(default)]
    r#type: String,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    placeholder: Option<String>,
    #[props(default)]
    onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let base = "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

    rsx! {
        input {
            class: "{base} {class}",
            r#type: "{props.r#type}",
            disabled: props.disabled,
            value: props.value.unwrap_or_default(),
            placeholder: props.placeholder.unwrap_or_default(),
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            }
        }
    }
}
