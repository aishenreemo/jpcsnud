use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        label {
            class: "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50 {class}",
            {props.children}
        }
    }
}
