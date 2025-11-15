use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    class: Option<String>,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    placeholder: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let base = "border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm";

    rsx! {
        textarea {
            class: "{base} {class}",
            value: props.value.unwrap_or_default(),
            placeholder: props.placeholder.unwrap_or_default(),
            disabled: props.disabled,
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            }
        }
    }
}
