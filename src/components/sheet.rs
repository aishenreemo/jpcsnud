use dioxus::prelude::*;
use lucide_rust::dioxus::x_icon::X;

#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let toggle = move |_| {
        if let Some(handler) = &props.on_open_change {
            handler.call(!props.open);
        }
    };

    rsx! {
        div {
            onclick: toggle,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetTriggerProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn SheetTrigger(props: SheetTriggerProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        button {
            class: "{class}",
            "data-slot": "sheet-trigger",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetContentProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    side: String,
}

#[component]
pub fn SheetContent(props: SheetContentProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let side_class = match props.side.as_str() {
        "left" => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm data-[state=open]:slide-in-from-left data-[state=closed]:slide-out-to-left",
        "top" => "inset-x-0 top-0 h-auto border-b data-[state=open]:slide-in-from-top data-[state=closed]:slide-out-to-top",
        "bottom" => "inset-x-0 bottom-0 h-auto border-t data-[state=open]:slide-in-from-bottom data-[state=closed]:slide-out-to-bottom",
        _ => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm data-[state=open]:slide-in-from-right data-[state=closed]:slide-out-to-right",
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50",
            div {
                class: "fixed inset-0 z-50 bg-black/50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0",
            }
            div {
                class: "bg-background fixed z-50 flex flex-col gap-4 shadow-lg transition ease-in-out data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:duration-500 data-[state=closed]:duration-300 {side_class} {class}",
                {props.children}
                button {
                    class: "ring-offset-background focus:ring-ring data-[state=open]:bg-secondary absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none",
                    X { class: "size-4" }
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetHeaderProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn SheetHeader(props: SheetHeaderProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-col gap-1.5 p-4 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetFooterProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn SheetFooter(props: SheetFooterProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "mt-auto flex flex-col gap-2 p-4 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetTitleProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn SheetTitle(props: SheetTitleProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        h2 {
            class: "text-foreground font-semibold {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetDescriptionProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn SheetDescription(props: SheetDescriptionProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        p {
            class: "text-muted-foreground text-sm {class}",
            {props.children}
        }
    }
}
