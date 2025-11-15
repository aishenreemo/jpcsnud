use dioxus::prelude::*;
use lucide_rust::dioxus::check_icon::Check;
use lucide_rust::dioxus::circle_icon::Circle;

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    rsx! { {props.children} }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    rsx! {
        button {
            "data-slot": "dropdown-menu-trigger",
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    class: Option<String>,
    visible: bool,
    children: Element,
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    if props.visible {
        rsx! {
            div {
                class: "bg-popover text-popover-foreground z-50 min-w-[8rem] rounded-md border p-1 shadow-md {class}",
                {props.children}
            }
        }
    } else {
        rsx!(Fragment {})
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    destructive: bool,
    #[props(default)]
    inset: bool,
}

#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let variant_class = if props.destructive {
        "text-destructive hover:bg-destructive/10 dark:hover:bg-destructive/20 hover:text-destructive"
    } else {
        "hover:bg-accent hover:text-accent-foreground"
    };
    let inset_class = if props.inset { "pl-8" } else { "" };

    rsx! {
        div {
            class: "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none select-none {variant_class} {inset_class} {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuCheckboxItemProps {
    class: Option<String>,
    children: Element,
    checked: bool,
}

#[component]
pub fn DropdownMenuCheckboxItem(props: DropdownMenuCheckboxItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-none select-none hover:bg-accent hover:text-accent-foreground {class}",
            span {
                class: "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center",
                if props.checked {
                    Check { class: "size-4" }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuRadioItemProps {
    class: Option<String>,
    children: Element,
    selected: bool,
}

#[component]
pub fn DropdownMenuRadioItem(props: DropdownMenuRadioItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-none select-none hover:bg-accent hover:text-accent-foreground {class}",
            span {
                class: "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center",
                if props.selected {
                    Circle { class: "size-2 fill-current" }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuLabelProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    inset: bool,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let inset_class = if props.inset { "pl-8" } else { "" };
    rsx! {
        div {
            class: "px-2 py-1.5 text-sm font-medium {inset_class} {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSeparatorProps {
    class: Option<String>,
}

#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "bg-border -mx-1 my-1 h-px {class}",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuShortcutProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DropdownMenuShortcut(props: DropdownMenuShortcutProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        span {
            class: "text-muted-foreground ml-auto text-xs tracking-widest {class}",
            {props.children}
        }
    }
}
