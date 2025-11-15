use dioxus::prelude::*;
use lucide_rust::dioxus::circle_check_icon::CircleCheck;
use lucide_rust::dioxus::info_icon::Info;
use lucide_rust::dioxus::loader_icon::Loader;
use lucide_rust::dioxus::octagon_x_icon::OctagonX;
use lucide_rust::dioxus::triangle_alert_icon::TriangleAlert;

#[derive(Props, Clone, PartialEq)]
pub struct ToasterProps {
    #[props(default)]
    theme: Option<String>,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Toaster(props: ToasterProps) -> Element {
    let theme = props.theme.clone().unwrap_or_else(|| "system".to_string());
    let class = props
        .class
        .clone()
        .unwrap_or_else(|| "toaster group".to_string());

    rsx! {
        div {
            class: "{class}",
            style: "--normal-bg: var(--popover); --normal-text: var(--popover-foreground); --normal-border: var(--border); --border-radius: var(--radius);",
            "data-theme": "{theme}",
            div { class: "flex items-center gap-2", CircleCheck { class: "size-4" } }
            div { class: "flex items-center gap-2", Info { class: "size-4" } }
            div { class: "flex items-center gap-2", TriangleAlert { class: "size-4" } }
            div { class: "flex items-center gap-2", OctagonX { class: "size-4" } }
            div { class: "flex items-center gap-2", Loader { class: "size-4 animate-spin" } }
        }
    }
}
