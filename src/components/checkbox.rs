use dioxus::prelude::*;
use lucide_rust::dioxus::check_icon::Check;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    class: Option<String>,
    #[props(default)]
    checked: bool,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let base = "peer border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50";

    rsx! {
        div {
            class: "{base} {class}",
            role: "checkbox",
            aria_checked: props.checked,
            tabindex: "0",
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            div {
                class: "grid place-content-center text-current transition-none",
                if props.checked {
                    Check {
                        class: "size-3.5",
                    }
                }
            }
        }
    }
}
