use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    #[props(default)]
    orientation: String,
    #[props(default)]
    decorative: bool,
    class: Option<String>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let orientation = props.orientation.as_str();
    let base = "bg-border shrink-0";
    let orientation_class = match orientation {
        "vertical" => "h-full w-px",
        _ => "h-px w-full",
    };

    rsx! {
        div {
            role: if props.decorative { "presentation" } else { "separator" },
            class: "{base} {orientation_class} {class}",
        }
    }
}
