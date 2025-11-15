use dioxus::prelude::*;
use lucide_rust::dioxus::x_icon::X;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    rsx! {
        {props.children}
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    children: Element,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    rsx! {
        button {
            "data-slot": "dialog-trigger",
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
pub struct DialogOverlayProps {
    class: Option<String>,
}

#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50 {class}",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    show_close_button: bool,
    #[props(default)]
    onclose: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "bg-background data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg duration-200 sm:max-w-lg {class}",
            {props.children}
            if props.show_close_button {
                button {
                    class: "ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-4 right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclose {
                            handler.call(evt);
                        }
                    },
                    X {
                        class: "size-4"
                    }
                    span {
                        class: "sr-only",
                        "Close"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-col gap-2 text-center sm:text-left {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        h2 {
            class: "text-lg leading-none font-semibold {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        p {
            class: "text-muted-foreground text-sm {class}",
            {props.children}
        }
    }
}
