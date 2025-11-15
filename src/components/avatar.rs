use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "relative flex size-8 shrink-0 overflow-hidden rounded-full {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    class: Option<String>,
    src: String,
    alt: String,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        img {
            class: "aspect-square size-full {class}",
            src: "{props.src}",
            alt: "{props.alt}",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "bg-muted flex size-full items-center justify-center rounded-full {class}",
            {props.children}
        }
    }
}
