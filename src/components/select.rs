use dioxus::prelude::*;
use lucide_rust::dioxus::check_icon::Check;
use lucide_rust::dioxus::chevron_down_icon::ChevronDown;
use lucide_rust::dioxus::chevron_up_icon::ChevronUp;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    rsx! { {props.children} }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    class: Option<String>,
    #[props(default)]
    size: String,
    children: Element,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let size_class = match props.size.as_str() {
        "sm" => "h-8",
        _ => "h-9",
    };

    rsx! {
        button {
            class: "border-input data-[placeholder]:text-muted-foreground [&_svg:not([class*='text-'])]:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 dark:hover:bg-input/50 flex w-fit items-center justify-between gap-2 rounded-md border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 {size_class} {class}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
            ChevronDown {
                class: "size-4 opacity-50"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectContentProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    position: String,
    #[props(default)]
    align: String,
}

#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let popper_class = if props.position == "popper" {
        "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1"
    } else {
        ""
    };

    rsx! {
        div {
            class: "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-50 max-h-[400px] min-w-[8rem] overflow-x-hidden overflow-y-auto rounded-md border shadow-md {popper_class} {class}",
            SelectScrollUpButton {}
            div {
                class: "p-1",
                {props.children}
            }
            SelectScrollDownButton {}
        }
    }
}

#[component]
pub fn SelectScrollUpButton() -> Element {
    rsx! {
        div {
            class: "flex cursor-default items-center justify-center py-1",
            ChevronUp {
                class: "size-4"
            }
        }
    }
}

#[component]
pub fn SelectScrollDownButton() -> Element {
    rsx! {
        div {
            class: "flex cursor-default items-center justify-center py-1",
            ChevronDown {
                class: "size-4"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectItemProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    selected: bool,
}

#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "focus:bg-accent focus:text-accent-foreground relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-none select-none [&_svg:not([class*='text-'])]:text-muted-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 {class}",
            if props.selected {
                span {
                    class: "absolute right-2 flex size-3.5 items-center justify-center",
                    Check {
                        class: "size-4"
                    }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectLabelProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "text-muted-foreground px-2 py-1.5 text-xs {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectSeparatorProps {
    class: Option<String>,
}

#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "bg-border pointer-events-none -mx-1 my-1 h-px {class}",
        }
    }
}
